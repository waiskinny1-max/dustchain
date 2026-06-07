pub type Result<T> = std::result::Result<T, MempoolError>;

#[derive(Debug, thiserror::Error)]
pub enum MempoolError {
    #[error("mempool rejected transaction: duplicate {0}")]
    Duplicate(dust_core::Hash),

    #[error("mempool rejected transaction: global limit reached")]
    GlobalLimit,

    #[error("mempool rejected transaction: account limit reached for {0}")]
    AccountLimit(dust_core::Address),
}
