use serde::{Deserialize, Serialize};

use crate::{Address, FeeBreakdown, FeePolicy, Hash};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u8,
    pub chain_id: String,
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub nonce: u64,
    pub max_fee: u64,
    pub priority_fee: u64,
    pub memo: Vec<u8>,
}

impl Transaction {
    pub fn transfer(
        chain_id: impl Into<String>,
        from: Address,
        to: Address,
        amount: u64,
        nonce: u64,
        max_fee: u64,
        priority_fee: u64,
        memo: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            version: 1,
            chain_id: chain_id.into(),
            from,
            to,
            amount,
            nonce,
            max_fee,
            priority_fee,
            memo: memo.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub tx: Transaction,
    pub public_key: [u8; 32],
    pub signature: [u8; 64],
    pub encoded_size: usize,
    pub hash: Hash,
}

impl SignedTransaction {
    pub fn new(tx: Transaction, public_key: [u8; 32], signature: [u8; 64], encoded_size: usize, hash: Hash) -> Self {
        Self { tx, public_key, signature, encoded_size, hash }
    }

    pub fn fee_breakdown(&self, policy: &FeePolicy) -> FeeBreakdown {
        policy.breakdown(self.encoded_size, self.tx.priority_fee)
    }

    pub fn paid_fee(&self, policy: &FeePolicy) -> u64 {
        self.fee_breakdown(policy).paid_fee
    }
}
