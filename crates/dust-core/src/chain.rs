
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{validate::validate_block, Address, Block, BlockHeader, DustError, FeePolicy, Result, SignedTransaction, State, ValidationContext};

#[derive(Clone, Debug)]
pub struct Chain {
    pub chain_id: String,
    pub blocks: Vec<Block>,
    pub genesis_state: State,
    pub state: State,
    pub policy: FeePolicy,
}

impl Chain {
    pub fn new(chain_id: impl Into<String>, policy: FeePolicy) -> Self {
        let chain_id = chain_id.into();
        let genesis_state = State::new();
        Self { blocks: vec![Block::genesis(chain_id.clone())], genesis_state: genesis_state.clone(), state: genesis_state, policy, chain_id }
    }

    pub fn with_genesis_state(chain_id: impl Into<String>, policy: FeePolicy, genesis_state: State) -> Self {
        let chain_id = chain_id.into();
        Self { blocks: vec![Block::genesis(chain_id.clone())], genesis_state: genesis_state.clone(), state: genesis_state, policy, chain_id }
    }

    pub fn height(&self) -> u64 {
        self.tip().map(|b| b.header.height).unwrap_or(0)
    }

    pub fn tip(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn mine<F>(&mut self, producer: Address, txs: Vec<SignedTransaction>, verifier: F) -> Result<Block>
    where
        F: Copy + Fn(&SignedTransaction) -> bool,
    {
        let parent = self.tip().cloned().ok_or(DustError::EmptyChain)?;
        let mut next_state = self.state.clone();
        for tx in &txs {
            if tx.tx.chain_id != self.chain_id {
                return Err(DustError::WrongChainId { expected: self.chain_id.clone(), received: tx.tx.chain_id.clone() });
            }
            next_state.apply_transaction(tx, &self.policy, verifier)?;
        }

        let block = Block {
            header: BlockHeader {
                version: 1,
                chain_id: self.chain_id.clone(),
                height: parent.header.height + 1,
                previous_block_hash: parent.header_hash(),
                state_root: next_state.root_hash(),
                tx_root: Block::tx_root(&txs),
                timestamp: now_secs(),
                producer,
                difficulty_or_slot: 0,
                nonce_or_round: 0,
            },
            transactions: txs,
        };

        self.state = next_state;
        self.blocks.push(block.clone());
        Ok(block)
    }

    pub fn verify<F>(&self, verifier: F) -> Result<()>
    where
        F: Copy + Fn(&SignedTransaction) -> bool,
    {
        if self.blocks.is_empty() {
            return Err(DustError::EmptyChain);
        }
        let ctx = ValidationContext { chain_id: self.chain_id.clone(), policy: self.policy, max_clock_drift_secs: 30 };
        let mut state = self.genesis_state.clone();
        for pair in self.blocks.windows(2) {
            let parent = &pair[0];
            let block = &pair[1];
            validate_block(parent, block, &state, &ctx, 0, verifier)?;
            for tx in &block.transactions {
                state.apply_transaction(tx, &self.policy, verifier)?;
            }
        }
        if state.root_hash() != self.state.root_hash() {
            return Err(DustError::StateRootMismatch { expected: state.root_hash(), received: self.state.root_hash() });
        }
        Ok(())
    }
}

pub fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}
