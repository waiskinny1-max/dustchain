pub type Result<T> = std::result::Result<T, NodeError>;

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("node error: local P2P is scheduled for v0.5; current command is a safe stub")]
    P2pNotImplemented,

    #[error("store error: {0}")]
    Store(#[from] dust_store::StoreError),
}
