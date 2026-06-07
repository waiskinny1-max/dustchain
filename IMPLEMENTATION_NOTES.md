# Implementation notes

This repository artifact was generated from the supplied `dustchain` specification.

## What is implemented

- Rust workspace with dedicated crates.
- Core account, transaction, fee, state, block, chain, validation, and Merkle modules.
- Ed25519 wallet key generation and signing helper crate.
- Custom `dust-wire` binary payloads and framed `.dtx` / `.dblk` files.
- CLI command surface for init, wallets, faucet, transfer, mining, balances, chain inspection, file inspection, mempool, benchmark scaffold, node stub, TUI snapshot, and local lab reports.
- File-backed local storage for the first implementation.
- Local-only lab simulation report modules.
- Documentation, examples, tests, CI, benches, and release hygiene files.

## Important limitation

I could not compile or run `cargo test` in this sandbox because `rustc` / `cargo` are not installed here. The code is written as a coherent Rust workspace, but you should run the following locally immediately after unzip:

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Any compiler issue after that should be treated as a normal first-pass patch, not as a tested release.

## Design decision

The fee policy keeps the first 1024 bytes inside the base fee. That resolves the spec tension between `base_fee + size_fee` and the desired demo result where a normal transfer pays only `1 dust`.
