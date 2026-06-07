pub type Result<T> = std::result::Result<T, StoreError>;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("wire error: {0}")]
    Wire(#[from] dust_wire::WireError),

    #[error("crypto error: {0}")]
    Crypto(#[from] dust_crypto::CryptoError),

    #[error("core error: {0}")]
    Core(#[from] dust_core::DustError),

    #[error("wallet not found: {0}")]
    WalletNotFound(String),

    #[error("store parse error: {0}")]
    Parse(String),
}
