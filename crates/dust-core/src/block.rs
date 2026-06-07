use serde::{Deserialize, Serialize};

use crate::{merkle::merkle_root, Address, Hash, SignedTransaction};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u8,
    pub chain_id: String,
    pub height: u64,
    pub previous_block_hash: Hash,
    pub state_root: Hash,
    pub tx_root: Hash,
    pub timestamp: u64,
    pub producer: Address,
    pub difficulty_or_slot: u64,
    pub nonce_or_round: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<SignedTransaction>,
}

impl Block {
    pub fn genesis(chain_id: impl Into<String>) -> Self {
        Self {
            header: BlockHeader {
                version: 1,
                chain_id: chain_id.into(),
                height: 0,
                previous_block_hash: Hash::ZERO,
                state_root: Hash::ZERO,
                tx_root: Hash::ZERO,
                timestamp: 0,
                producer: Address::ZERO,
                difficulty_or_slot: 0,
                nonce_or_round: 0,
            },
            transactions: Vec::new(),
        }
    }

    pub fn tx_root(transactions: &[SignedTransaction]) -> Hash {
        merkle_root(transactions.iter().map(|tx| tx.hash).collect())
    }

    pub fn header_hash(&self) -> Hash {
        let mut bytes = Vec::new();
        bytes.push(self.header.version);
        push_str(&mut bytes, &self.header.chain_id);
        bytes.extend_from_slice(&self.header.height.to_le_bytes());
        bytes.extend_from_slice(self.header.previous_block_hash.as_bytes());
        bytes.extend_from_slice(self.header.state_root.as_bytes());
        bytes.extend_from_slice(self.header.tx_root.as_bytes());
        bytes.extend_from_slice(&self.header.timestamp.to_le_bytes());
        bytes.extend_from_slice(self.header.producer.as_bytes());
        bytes.extend_from_slice(&self.header.difficulty_or_slot.to_le_bytes());
        bytes.extend_from_slice(&self.header.nonce_or_round.to_le_bytes());
        Hash::digest(bytes)
    }
}

fn push_str(out: &mut Vec<u8>, value: &str) {
    out.extend_from_slice(&(value.len() as u64).to_le_bytes());
    out.extend_from_slice(value.as_bytes());
}
