use std::collections::{BTreeMap, HashMap};

use dust_core::{Address, FeePolicy, Hash, SignedTransaction};

use crate::{MempoolError, MempoolPolicy, Result, TxPriority};

#[derive(Clone, Debug)]
pub struct Mempool {
    policy: MempoolPolicy,
    txs: BTreeMap<Hash, SignedTransaction>,
    per_account: HashMap<Address, usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MempoolStats {
    pub txs: usize,
    pub accounts: usize,
}

impl Mempool {
    pub fn new(policy: MempoolPolicy) -> Self {
        Self { policy, txs: BTreeMap::new(), per_account: HashMap::new() }
    }

    pub fn insert(&mut self, tx: SignedTransaction) -> Result<()> {
        if self.txs.contains_key(&tx.hash) {
            return Err(MempoolError::Duplicate(tx.hash));
        }
        if self.txs.len() >= self.policy.max_txs {
            return Err(MempoolError::GlobalLimit);
        }
        let count = self.per_account.get(&tx.tx.from).copied().unwrap_or(0);
        if count >= self.policy.max_txs_per_account {
            return Err(MempoolError::AccountLimit(tx.tx.from));
        }
        self.per_account.insert(tx.tx.from, count + 1);
        self.txs.insert(tx.hash, tx);
        Ok(())
    }

    pub fn ordered_by_fee(&self, fees: &FeePolicy) -> Vec<&SignedTransaction> {
        let mut txs: Vec<_> = self.txs.values().collect();
        txs.sort_by_key(|tx| std::cmp::Reverse(TxPriority::from_tx(tx, fees).fee_per_byte_microunits));
        txs
    }

    pub fn remove(&mut self, hash: &Hash) -> Option<SignedTransaction> {
        let tx = self.txs.remove(hash)?;
        if let Some(count) = self.per_account.get_mut(&tx.tx.from) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                self.per_account.remove(&tx.tx.from);
            }
        }
        Some(tx)
    }

    pub fn stats(&self) -> MempoolStats {
        MempoolStats { txs: self.txs.len(), accounts: self.per_account.len() }
    }
}
