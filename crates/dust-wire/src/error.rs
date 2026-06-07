pub type Result<T> = std::result::Result<T, WireError>;

#[derive(Debug, thiserror::Error)]
pub enum WireError {
    #[error("wire decode failed: unexpected end of input")]
    UnexpectedEof,

    #[error("wire decode failed: varint is too large")]
    VarintTooLarge,

    #[error("wire decode failed: invalid magic; expected {expected}, received {received}")]
    InvalidMagic { expected: String, received: String },

    #[error("wire decode failed: unsupported version {0}")]
    UnsupportedVersion(u8),

    #[error("wire decode failed: checksum mismatch")]
    BadChecksum,

    #[error("wire decode failed: trailing bytes: {0}")]
    TrailingBytes(usize),

    #[error("wire decode failed: invalid utf8")]
    InvalidUtf8,

    #[error("wire decode failed: invalid address bytes")]
    InvalidAddress,

    #[error("wire decode failed: invalid hash bytes")]
    InvalidHash,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
