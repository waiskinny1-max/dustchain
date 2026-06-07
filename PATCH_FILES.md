# dustchain v0.4 patch files

Unzip this archive at the root of your existing `dustchain` repository.
It contains only files that are new or changed since the previous v0.1-v0.3 bundle.

## Files

- `CHANGELOG.md`
- `Cargo.toml`
- `README.md`
- `benches/block_validation.rs`
- `benches/fee_calculation.rs`
- `benches/mempool_sorting.rs`
- `benches/storage_write.rs`
- `benches/tx_encoding.rs`
- `crates/dust-cli/Cargo.toml`
- `crates/dust-cli/src/main.rs`
- `crates/dust-core/Cargo.toml`
- `crates/dust-crypto/Cargo.toml`
- `crates/dust-lab/Cargo.toml`
- `crates/dust-mempool/Cargo.toml`
- `crates/dust-node/Cargo.toml`
- `crates/dust-store/Cargo.toml`
- `crates/dust-store/src/blocks.rs`
- `crates/dust-store/src/db.rs`
- `crates/dust-store/src/error.rs`
- `crates/dust-store/src/lib.rs`
- `crates/dust-store/src/metadata.rs`
- `crates/dust-store/src/state.rs`
- `crates/dust-store/src/txs.rs`
- `crates/dust-tui/Cargo.toml`
- `crates/dust-wire/Cargo.toml`
- `docs/benchmarks.md`
- `docs/protocol.md`
- `docs/roadmap.md`
- `docs/storage.md`
- `tests/localnet_sync.rs`
- `tests/storage_persistence.rs`

## Local validation

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
