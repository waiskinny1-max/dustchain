//! Local-first TCP control protocol for `dustchain` v0.5.
//!
//! The protocol is intentionally narrow: line-delimited commands, bounded
//! frames, loopback-first defaults, and no third-party targeting behavior.
//! This is for local development nodes, not public-network deployment.

use std::{net::IpAddr, time::Duration};

use dust_core::Hash;
use dust_store::DustStore;
use dust_wire::{block_file_bytes, decode::decode_signed_tx_file};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    time::timeout,
};

use crate::{NodeConfig, NodeError, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerStatus {
    pub chain_id: String,
    pub height: u64,
    pub tip_hash: Hash,
    pub mempool_txs: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PeerRequest {
    Hello { chain_id: String, height: u64, tip_hash: Hash },
    Status,
    GetBlock { height: u64 },
    GetMempool,
    SubmitTx { file_hex: String },
    Ping,
}

impl PeerRequest {
    pub fn parse(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace();
        let Some(kind) = parts.next() else {
            return Err(NodeError::Protocol("empty peer message".to_string()));
        };

        match kind {
            "HELLO" => {
                let chain_id = parts.next().ok_or_else(|| NodeError::Protocol("HELLO missing chain_id".to_string()))?.to_string();
                let height = parse_u64(parts.next(), "HELLO height")?;
                let tip_hash = parse_hash(parts.next(), "HELLO tip_hash")?;
                Ok(Self::Hello { chain_id, height, tip_hash })
            }
            "STATUS" => Ok(Self::Status),
            "GET_BLOCK" => Ok(Self::GetBlock { height: parse_u64(parts.next(), "GET_BLOCK height")? }),
            "GET_MEMPOOL" => Ok(Self::GetMempool),
            "SUBMIT_TX" => Ok(Self::SubmitTx { file_hex: parts.next().ok_or_else(|| NodeError::Protocol("SUBMIT_TX missing hex payload".to_string()))?.to_string() }),
            "PING" => Ok(Self::Ping),
            other => Err(NodeError::Protocol(format!("unknown peer message: {other}"))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LocalClient {
    max_frame_bytes: usize,
    timeout: Duration,
}

impl LocalClient {
    pub fn new(max_frame_bytes: usize, timeout_ms: u64) -> Self {
        Self { max_frame_bytes, timeout: Duration::from_millis(timeout_ms) }
    }

    pub async fn status(&self, addr: &str) -> Result<PeerStatus> {
        let line = self.request(addr, "STATUS").await?;
        parse_status_response(&line)
    }

    pub async fn fetch_block(&self, addr: &str, height: u64) -> Result<Option<Vec<u8>>> {
        let line = self.request(addr, &format!("GET_BLOCK {height}")).await?;
        if line.starts_with("NOT_FOUND ") {
            return Ok(None);
        }
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next(), parts.next()) {
            (Some("BLOCK"), Some(_height), Some(hex_bytes)) => Ok(Some(hex::decode(hex_bytes)?)),
            _ => Err(NodeError::Protocol(format!("unexpected GET_BLOCK response: {line}"))),
        }
    }

    pub async fn submit_tx_file(&self, addr: &str, file_bytes: &[u8]) -> Result<String> {
        let line = self.request(addr, &format!("SUBMIT_TX {}", hex::encode(file_bytes))).await?;
        if line.starts_with("ACCEPTED_TX ") {
            Ok(line)
        } else {
            Err(NodeError::Protocol(format!("unexpected SUBMIT_TX response: {line}")))
        }
    }

    pub async fn ping(&self, addr: &str) -> Result<()> {
        let line = self.request(addr, "PING").await?;
        if line.trim() == "PONG" {
            Ok(())
        } else {
            Err(NodeError::Protocol(format!("unexpected ping response: {line}")))
        }
    }

    async fn request(&self, addr: &str, request: &str) -> Result<String> {
        let mut stream = timeout(self.timeout, TcpStream::connect(addr))
            .await
            .map_err(|_| NodeError::Timeout(format!("connect {addr}")))??;
        let outbound = format!("{request}\n");
        if outbound.len() > self.max_frame_bytes {
            return Err(NodeError::FrameTooLarge { max: self.max_frame_bytes, received: outbound.len() });
        }
        timeout(self.timeout, stream.write_all(outbound.as_bytes()))
            .await
            .map_err(|_| NodeError::Timeout(format!("write {addr}")))??;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        let read = timeout(self.timeout, reader.read_line(&mut line))
            .await
            .map_err(|_| NodeError::Timeout(format!("read {addr}")))??;
        if read == 0 {
            return Err(NodeError::Protocol("peer closed without response".to_string()));
        }
        if line.len() > self.max_frame_bytes {
            return Err(NodeError::FrameTooLarge { max: self.max_frame_bytes, received: line.len() });
        }
        Ok(line.trim_end().to_string())
    }
}

pub async fn handle_peer(stream: TcpStream, store: DustStore, max_frame_bytes: usize, invalid_message_limit: usize) -> Result<()> {
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);
    let mut invalid_messages = 0usize;

    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line).await?;
        if read == 0 {
            return Ok(());
        }
        if line.len() > max_frame_bytes {
            write_half.write_all(format!("ERROR frame_too_large max={max_frame_bytes} received={}\n", line.len()).as_bytes()).await?;
            return Err(NodeError::FrameTooLarge { max: max_frame_bytes, received: line.len() });
        }

        let response = match PeerRequest::parse(line.trim_end()) {
            Ok(request) => match process_request(request, &store) {
                Ok(response) => response,
                Err(err) => format!("ERROR {err}\n"),
            },
            Err(err) => {
                invalid_messages += 1;
                if invalid_messages >= invalid_message_limit {
                    write_half.write_all(b"ERROR invalid_message_limit\n").await?;
                    return Ok(());
                }
                format!("ERROR {err}\n")
            }
        };
        write_half.write_all(response.as_bytes()).await?;
    }
}

pub fn process_request(request: PeerRequest, store: &DustStore) -> Result<String> {
    match request {
        PeerRequest::Hello { .. } | PeerRequest::Status => render_status(store),
        PeerRequest::Ping => Ok("PONG\n".to_string()),
        PeerRequest::GetBlock { height } => render_block(store, height),
        PeerRequest::GetMempool => render_mempool(store),
        PeerRequest::SubmitTx { file_hex } => accept_tx(store, &file_hex),
    }
}

pub fn render_status(store: &DustStore) -> Result<String> {
    let cfg = store.config()?;
    let chain = store.load_chain()?;
    let tip_hash = chain.tip().map(|block| block.header_hash()).unwrap_or(Hash::ZERO);
    let mempool_txs = store.pending_txs()?.len();
    Ok(format!("STATUS {} {} {} {}\n", cfg.chain_id, chain.height(), tip_hash, mempool_txs))
}

pub fn validate_loopback_bind(config: &NodeConfig) -> Result<()> {
    if config.allow_non_loopback {
        return Ok(());
    }
    if config.host == "localhost" {
        return Ok(());
    }
    let ip: IpAddr = config
        .host
        .parse()
        .map_err(|_| NodeError::NonLoopbackHost { host: config.host.clone() })?;
    if ip.is_loopback() {
        Ok(())
    } else {
        Err(NodeError::NonLoopbackHost { host: config.host.clone() })
    }
}

fn render_block(store: &DustStore, height: u64) -> Result<String> {
    let blocks = store.load_blocks_ordered()?;
    let Some((_, block)) = blocks.into_iter().find(|(_, block)| block.header.height == height) else {
        return Ok(format!("NOT_FOUND block {height}\n"));
    };
    Ok(format!("BLOCK {} {}\n", height, hex::encode(block_file_bytes(&block))))
}

fn render_mempool(store: &DustStore) -> Result<String> {
    let pending = store.pending_txs()?;
    let hashes = pending.into_iter().map(|(_, tx)| tx.hash.to_string()).collect::<Vec<_>>().join(",");
    Ok(format!("MEMPOOL {} {}\n", hashes.split(',').filter(|v| !v.is_empty()).count(), hashes))
}

fn accept_tx(store: &DustStore, file_hex: &str) -> Result<String> {
    let bytes = hex::decode(file_hex)?;
    let signed = decode_signed_tx_file(&bytes)?;
    let path = store.save_pending_tx(&signed)?;
    Ok(format!("ACCEPTED_TX {} file={}\n", signed.hash, path.display()))
}

fn parse_status_response(line: &str) -> Result<PeerStatus> {
    let mut parts = line.split_whitespace();
    if parts.next() != Some("STATUS") {
        return Err(NodeError::Protocol(format!("expected STATUS response, received: {line}")));
    }
    let chain_id = parts.next().ok_or_else(|| NodeError::Protocol("STATUS missing chain_id".to_string()))?.to_string();
    let height = parse_u64(parts.next(), "STATUS height")?;
    let tip_hash = parse_hash(parts.next(), "STATUS tip_hash")?;
    let mempool_txs = parse_usize(parts.next(), "STATUS mempool_txs")?;
    Ok(PeerStatus { chain_id, height, tip_hash, mempool_txs })
}

fn parse_u64(value: Option<&str>, field: &str) -> Result<u64> {
    value
        .ok_or_else(|| NodeError::Protocol(format!("{field} missing")))?
        .parse()
        .map_err(|_| NodeError::Protocol(format!("{field} is not a number")))
}

fn parse_usize(value: Option<&str>, field: &str) -> Result<usize> {
    value
        .ok_or_else(|| NodeError::Protocol(format!("{field} missing")))?
        .parse()
        .map_err(|_| NodeError::Protocol(format!("{field} is not a number")))
}

fn parse_hash(value: Option<&str>, field: &str) -> Result<Hash> {
    value
        .ok_or_else(|| NodeError::Protocol(format!("{field} missing")))?
        .parse()
        .map_err(|_| NodeError::Protocol(format!("{field} is not a valid hash")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_status_request() {
        assert_eq!(PeerRequest::parse("STATUS").unwrap(), PeerRequest::Status);
    }

    #[test]
    fn rejects_unknown_request() {
        let err = PeerRequest::parse("NUKE 127.0.0.1").unwrap_err().to_string();
        assert!(err.contains("unknown peer message"));
    }
}
