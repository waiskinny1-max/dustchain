use dust_core::{Block, Hash, SignedTransaction, Transaction};

use crate::{magic::{BLOCK_MAGIC, TX_MAGIC, WIRE_VERSION}, varint::{put_bytes, put_varint}};

pub fn transaction_signing_payload(tx: &Transaction) -> Vec<u8> {
    let mut out = Vec::with_capacity(192 + tx.memo.len());
    put_transaction(&mut out, tx);
    out
}

pub fn signed_tx_payload(signed: &SignedTransaction) -> Vec<u8> {
    let mut out = Vec::with_capacity(300 + signed.tx.memo.len());
    put_transaction(&mut out, &signed.tx);
    out.extend_from_slice(&signed.public_key);
    out.extend_from_slice(&signed.signature);
    out
}

pub fn signed_tx_file_bytes(signed: &SignedTransaction) -> Vec<u8> {
    let payload = signed_tx_payload(signed);
    let mut out = Vec::with_capacity(TX_MAGIC.len() + payload.len() + 40);
    out.extend_from_slice(TX_MAGIC);
    out.push(WIRE_VERSION);
    put_varint(&mut out, payload.len() as u64);
    out.extend_from_slice(&payload);
    let checksum = Hash::digest(&out);
    out.extend_from_slice(checksum.as_bytes());
    out
}

pub fn block_payload(block: &Block) -> Vec<u8> {
    let mut out = Vec::new();
    put_block_header(&mut out, block);
    put_varint(&mut out, block.transactions.len() as u64);
    for tx in &block.transactions {
        let payload = signed_tx_payload(tx);
        put_bytes(&mut out, &payload);
    }
    out
}

pub fn block_file_bytes(block: &Block) -> Vec<u8> {
    let payload = block_payload(block);
    let mut out = Vec::with_capacity(BLOCK_MAGIC.len() + payload.len() + 40);
    out.extend_from_slice(BLOCK_MAGIC);
    out.push(WIRE_VERSION);
    put_varint(&mut out, payload.len() as u64);
    out.extend_from_slice(&payload);
    let checksum = Hash::digest(&out);
    out.extend_from_slice(checksum.as_bytes());
    out
}

fn put_transaction(out: &mut Vec<u8>, tx: &Transaction) {
    out.push(tx.version);
    put_bytes(out, tx.chain_id.as_bytes());
    out.extend_from_slice(tx.from.as_bytes());
    out.extend_from_slice(tx.to.as_bytes());
    out.extend_from_slice(&tx.amount.to_le_bytes());
    out.extend_from_slice(&tx.nonce.to_le_bytes());
    out.extend_from_slice(&tx.max_fee.to_le_bytes());
    out.extend_from_slice(&tx.priority_fee.to_le_bytes());
    put_bytes(out, &tx.memo);
}

fn put_block_header(out: &mut Vec<u8>, block: &Block) {
    out.push(block.header.version);
    put_bytes(out, block.header.chain_id.as_bytes());
    out.extend_from_slice(&block.header.height.to_le_bytes());
    out.extend_from_slice(block.header.previous_block_hash.as_bytes());
    out.extend_from_slice(block.header.state_root.as_bytes());
    out.extend_from_slice(block.header.tx_root.as_bytes());
    out.extend_from_slice(&block.header.timestamp.to_le_bytes());
    out.extend_from_slice(block.header.producer.as_bytes());
    out.extend_from_slice(&block.header.difficulty_or_slot.to_le_bytes());
    out.extend_from_slice(&block.header.nonce_or_round.to_le_bytes());
}
