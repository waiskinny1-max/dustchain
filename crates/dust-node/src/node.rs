use dust_core::Hash;
use dust_store::DustStore;
use tokio::net::TcpListener;
use tracing::{error, info, warn};

use crate::{p2p, LocalClient, NodeConfig, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeStatus {
    pub listening_on: String,
    pub height: u64,
    pub tip_hash: Hash,
    pub mempool_txs: usize,
    pub p2p_enabled: bool,
    pub configured_peers: usize,
}

pub struct Node {
    config: NodeConfig,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        Self { config }
    }

    pub async fn start_local_stub(&self) -> Result<NodeStatus> {
        let store = DustStore::open(&self.config.data_dir);
        let chain = store.load_chain()?;
        let mempool_txs = store.pending_txs()?.len();
        Ok(NodeStatus {
            listening_on: self.config.bind_addr(),
            height: chain.height(),
            tip_hash: chain.tip().map(|block| block.header_hash()).unwrap_or(Hash::ZERO),
            mempool_txs,
            p2p_enabled: false,
            configured_peers: 0,
        })
    }

    pub async fn local_status(&self, configured_peers: usize) -> Result<NodeStatus> {
        let store = DustStore::open(&self.config.data_dir);
        let chain = store.load_chain()?;
        let mempool_txs = store.pending_txs()?.len();
        Ok(NodeStatus {
            listening_on: self.config.bind_addr(),
            height: chain.height(),
            tip_hash: chain.tip().map(|block| block.header_hash()).unwrap_or(Hash::ZERO),
            mempool_txs,
            p2p_enabled: true,
            configured_peers,
        })
    }

    pub async fn start_localnet(&self, peers: Vec<String>) -> Result<()> {
        p2p::validate_loopback_bind(&self.config)?;
        let addr = self.config.bind_addr();
        let listener = TcpListener::bind(&addr).await?;
        let store = DustStore::open(&self.config.data_dir);
        let client = LocalClient::new(self.config.max_frame_bytes, self.config.connect_timeout_ms);

        info!(%addr, peers = peers.len(), "dustchain localnet listener started");
        for peer in peers {
            let client = client.clone();
            tokio::spawn(async move {
                match client.status(&peer).await {
                    Ok(status) => info!(%peer, height = status.height, mempool_txs = status.mempool_txs, "peer reachable"),
                    Err(err) => warn!(%peer, %err, "peer probe failed"),
                }
            });
        }

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            let store = store.clone();
            let max_frame_bytes = self.config.max_frame_bytes;
            let invalid_message_limit = self.config.invalid_message_limit;
            tokio::spawn(async move {
                if let Err(err) = p2p::handle_peer(stream, store, max_frame_bytes, invalid_message_limit).await {
                    error!(%peer_addr, %err, "peer session ended with error");
                }
            });
        }
    }
}
