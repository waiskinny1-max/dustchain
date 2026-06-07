use std::collections::HashSet;

use crate::{Block, DustError, FeePolicy, Hash, Result, State};

#[derive(Clone, Debug)]
pub struct ValidationContext {
    pub chain_id: String,
    pub policy: FeePolicy,
    pub max_clock_drift_secs: u64,
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self { chain_id: "dust-local".to_string(), policy: FeePolicy::default(), max_clock_drift_secs: 30 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReport {
    pub height: u64,
    pub txs: usize,
    pub total_fees: u64,
    pub state_root: Hash,
    pub tx_root: Hash,
}

pub fn validate_block<F>(
    parent: &Block,
    block: &Block,
    pre_state: &State,
    ctx: &ValidationContext,
    encoded_size: usize,
    verifier: F,
) -> Result<ValidationReport>
where
    F: Copy + Fn(&crate::SignedTransaction) -> bool,
{
    if block.header.chain_id != ctx.chain_id {
        return Err(DustError::WrongChainId { expected: ctx.chain_id.clone(), received: block.header.chain_id.clone() });
    }
    if block.header.height != parent.header.height + 1 {
        return Err(DustError::WrongHeight { expected: parent.header.height + 1, received: block.header.height });
    }
    let expected_prev = parent.header_hash();
    if block.header.previous_block_hash != expected_prev {
        return Err(DustError::PreviousHashMismatch { expected: expected_prev, received: block.header.previous_block_hash });
    }
    if encoded_size > ctx.policy.max_block_size_bytes {
        return Err(DustError::BlockTooLarge { max: ctx.policy.max_block_size_bytes, received: encoded_size });
    }

    let expected_tx_root = Block::tx_root(&block.transactions);
    if expected_tx_root != block.header.tx_root {
        return Err(DustError::TxRootMismatch { expected: expected_tx_root, received: block.header.tx_root });
    }

    let mut seen = HashSet::new();
    let mut replay_state = pre_state.clone();
    let mut total_fees = 0;
    for tx in &block.transactions {
        if !seen.insert(tx.hash) {
            return Err(DustError::DuplicateTransaction(tx.hash));
        }
        if tx.tx.chain_id != ctx.chain_id {
            return Err(DustError::WrongChainId { expected: ctx.chain_id.clone(), received: tx.tx.chain_id.clone() });
        }
        total_fees += replay_state.apply_transaction(tx, &ctx.policy, verifier)?;
    }

    let expected_state_root = replay_state.root_hash();
    if expected_state_root != block.header.state_root {
        return Err(DustError::StateRootMismatch { expected: expected_state_root, received: block.header.state_root });
    }

    Ok(ValidationReport { height: block.header.height, txs: block.transactions.len(), total_fees, state_root: expected_state_root, tx_root: expected_tx_root })
}
