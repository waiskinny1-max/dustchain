//! Pending transaction persistence boundary.
//!
//! Pending transactions are stored as `.dtx` files under `.dustchain/mempool/`.
//! The files remain binary-inspectable and are removed after successful mining.
