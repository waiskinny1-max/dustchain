use dust_core::{FeePolicy, Hash, SignedTransaction};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxPriority {
    pub hash: Hash,
    pub fee_per_byte_microunits: u64,
    pub paid_fee: u64,
    pub encoded_size: usize,
}

impl TxPriority {
    pub fn from_tx(tx: &SignedTransaction, fees: &FeePolicy) -> Self {
        let breakdown = tx.fee_breakdown(fees);
        Self {
            hash: tx.hash,
            fee_per_byte_microunits: breakdown.fee_per_byte_microunits,
            paid_fee: breakdown.paid_fee,
            encoded_size: breakdown.encoded_size,
        }
    }
}
