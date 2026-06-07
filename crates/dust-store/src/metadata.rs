//! Metadata helpers for the v0.4 persistent store.
//!
//! The concrete implementation lives in `db.rs` to keep the public API compact:
//! `StoreMetadata`, `BlockIndexEntry`, and `StoreStats` are exported from the
//! store crate and written to `.dustchain/metadata/`.
