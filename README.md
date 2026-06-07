# dustchain

Low fees. Compact blocks. Terminal-first.

`dustchain` is an experimental low-fee blockchain implementation built in Rust. It focuses on compact binary transactions, predictable micro-fees, local-first chain operation, persistent block/state storage, terminal inspection, and safe local adversarial simulations.

It is a protocol engineering portfolio project. It is not a production cryptocurrency, investment product, or public network.

## Why

Low blockchain fees do not come from “cheap mining.” They come from engineering choices:

- compact transaction encoding;
- deterministic fee rules;
- limited execution complexity;
- bounded transaction and block sizes;
- disciplined mempool policy;
- visible storage and benchmark costs.

The design target is simple: normal transfers should remain small, measurable, and cheap without making spam free.

## Current release

This repo is now at **v0.4.0-storage**.

Implemented foundation:

- account state with balances and nonces;
- Ed25519-style signed transfers;
- deterministic base fee + size fee + priority fee;
- binary `.dtx` transaction files;
- binary `.dblk` block files;
- CLI wallet, transfer, mining, balance, chain verification, file inspection;
- file-backed persistent local store;
- metadata manifest and block index;
- atomic state/config/wallet/block writes;
- DB stats and state export commands;
- safe local lab stubs.

## Quick start

```bash
cargo run -p dust-cli -- init
cargo run -p dust-cli -- wallet new alice
cargo run -p dust-cli -- wallet new bob
cargo run -p dust-cli -- faucet alice 1000
cargo run -p dust-cli -- tx send alice bob 100
cargo run -p dust-cli -- mine
cargo run -p dust-cli -- balance alice
cargo run -p dust-cli -- balance bob
cargo run -p dust-cli -- chain verify
```

Expected economic result:

```text
alice balance: 899 dust
bob balance: 100 dust
fee paid: 1 dust
chain status: valid
```

## Low-fee model

```text
required_fee = base_fee + size_fee
paid_fee = required_fee + priority_fee
```

Default policy:

```toml
[fees]
base_fee = 1
fee_per_kb = 1
included_bytes = 1024
max_priority_fee = 1000
```

Normal transfers fit under the included byte window, so the minimum transfer fee remains `1 dust`. Larger transactions pay a deterministic size surcharge.

## Storage layout

After `dust init`, the local store is created under `.dustchain/`:

```text
.dustchain/
├── config.toml
├── genesis.snapshot
├── state.snapshot
├── blocks/
│   └── 00000001.dblk
├── mempool/
│   └── <tx>.dtx
├── metadata/
│   ├── manifest.txt
│   └── block-index.tsv
└── wallets/
    └── alice.wallet
```

v0.4 adds:

- atomic file writes;
- persistent metadata manifest;
- block index regeneration;
- richer storage statistics;
- state export.

Useful commands:

```bash
cargo run -p dust-cli -- chain db-stats --verbose
cargo run -p dust-cli -- chain reindex
cargo run -p dust-cli -- chain export-state
cargo run -p dust-cli -- chain export-state --output state.txt
```

## Binary inspection

```bash
cargo run -p dust-cli -- inspect tx .dustchain/mempool/<tx>.dtx
cargo run -p dust-cli -- inspect block .dustchain/blocks/00000001.dblk
```

Example block output:

```text
Block File: .dustchain/blocks/00000001.dblk
Magic: DUSTBLK
Version: 1
Height: 1
Transactions: 1
Checksum: valid
```

## Benchmarks

Run:

```bash
cargo run -p dust-cli -- bench
cargo run -p dust-cli -- bench --markdown > BENCHMARKS.md
```

Do not commit fake benchmark numbers. Generate them from your machine and paste the real output.

## Local lab

The lab commands are local-only and do not target third-party systems:

```bash
cargo run -p dust-cli -- lab spam --txs 50000
cargo run -p dust-cli -- lab replay
cargo run -p dust-cli -- lab invalid-tx
cargo run -p dust-cli -- lab invalid-block
cargo run -p dust-cli -- lab oversized-block
cargo run -p dust-cli -- lab fork
```

## Roadmap

- v0.1 core chain
- v0.2 fee engine
- v0.3 binary format
- v0.4 persistent storage
- v0.5 local P2P networking
- v0.6 terminal UI
- v0.7 benchmark suite
- v0.8 local adversarial lab
- v1.0 portfolio release

## Security

This is not production-ready. It has not been externally audited. Do not use it for real funds, public consensus, or production security decisions.
