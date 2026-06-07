use crate::{Address, Hash};

pub type Result<T> = std::result::Result<T, DustError>;

#[derive(Debug, thiserror::Error)]
pub enum DustError {
    #[error("invalid address length: expected 32 bytes, received {received}")]
    InvalidAddressLength { received: usize },

    #[error("invalid hash length: expected 32 bytes, received {received}")]
    InvalidHashLength { received: usize },

    #[error("transaction rejected: wrong chain id; expected {expected}, received {received}")]
    WrongChainId { expected: String, received: String },

    #[error("transaction rejected: zero amount")]
    ZeroAmount,

    #[error("transaction rejected: memo too large; max {max} bytes, received {received} bytes")]
    MemoTooLarge { max: usize, received: usize },

    #[error("transaction rejected: encoded transaction too large; max {max} bytes, received {received} bytes")]
    TransactionTooLarge { max: usize, received: usize },

    #[error("transaction rejected: priority fee too high; max {max}, received {received}")]
    PriorityFeeTooHigh { max: u64, received: u64 },

    #[error("transaction rejected: insufficient max fee; required {required}, received {received}")]
    InsufficientMaxFee { required: u64, received: u64 },

    #[error("transaction rejected: nonce mismatch for {address}; expected {expected}, received {received}")]
    NonceMismatch { address: Address, expected: u64, received: u64 },

    #[error("transaction rejected: insufficient balance for {address}; required {required}, available {available}")]
    InsufficientBalance { address: Address, required: u64, available: u64 },

    #[error("transaction rejected: signature verification failed")]
    BadSignature,

    #[error("block rejected: wrong height; expected {expected}, received {received}")]
    WrongHeight { expected: u64, received: u64 },

    #[error("block rejected: previous hash mismatch; expected {expected}, received {received}")]
    PreviousHashMismatch { expected: Hash, received: Hash },

    #[error("block rejected: transaction root mismatch; expected {expected}, received {received}")]
    TxRootMismatch { expected: Hash, received: Hash },

    #[error("block rejected: state root mismatch; expected {expected}, received {received}")]
    StateRootMismatch { expected: Hash, received: Hash },

    #[error("block rejected: duplicate transaction {0}")]
    DuplicateTransaction(Hash),

    #[error("block rejected: block too large; max {max} bytes, received {received} bytes")]
    BlockTooLarge { max: usize, received: usize },

    #[error("chain rejected: empty chain")]
    EmptyChain,
}
