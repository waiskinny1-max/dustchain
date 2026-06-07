//! Block persistence boundary.
//!
//! v0.4 stores blocks as checksum-protected `.dblk` files and maintains a
//! tab-separated block index at `.dustchain/metadata/block-index.tsv`.
//! This keeps the storage layer inspectable before any future embedded-DB move.
