//! State persistence boundary.
//!
//! v0.4 writes account state snapshots atomically to avoid partially-written
//! state files during local demos or interrupted commands.
