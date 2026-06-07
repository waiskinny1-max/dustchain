pub type Result<T> = std::result::Result<T, NodeError>;

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("store error: {0}")]
    Store(#[from] dust_store::StoreError),

    #[error("wire error: {0}")]
    Wire(#[from] dust_wire::WireError),

    #[error("hex error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("protocol error: {0}")]
    Protocol(String),

    #[error("frame too large: max={max} received={received}")]
    FrameTooLarge { max: usize, received: usize },

    #[error("refusing non-loopback bind host `{host}`; pass allow_non_loopback=true only for controlled labs")]
    NonLoopbackHost { host: String },

    #[error("peer timeout: {0}")]
    Timeout(String),
}
