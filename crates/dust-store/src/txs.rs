//! Focused storage helpers for `txs`.
//!
//! The first implementation keeps these modules intentionally thin and routes
//! concrete file operations through `DustStore`. The split is here to keep the
//! workspace structure stable as the storage layer moves toward `redb` in v0.4.
