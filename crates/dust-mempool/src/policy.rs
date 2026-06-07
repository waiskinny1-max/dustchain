#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MempoolPolicy {
    pub max_txs: usize,
    pub max_txs_per_account: usize,
    pub max_txs_per_peer: usize,
}

impl Default for MempoolPolicy {
    fn default() -> Self {
        Self { max_txs: 10_000, max_txs_per_account: 64, max_txs_per_peer: 512 }
    }
}
