# dustchain

Low fees. Compact blocks. Terminal-first.

`dustchain` is an experimental low-fee blockchain implementation built in Rust. It focuses on compact binary transactions, deterministic micro-fees, local validation, mempool discipline, binary block inspection, and a terminal-native operator experience.

It is not a cryptocurrency launch. It is not investment software. It is a systems/security engineering repository.

## Why

Low fees are not created by cheap mining. They come from protocol decisions:

- compact transaction encoding;
- deterministic fee policy;
- small execution surface;
- strict transaction and block size limits;
- nonce-based replay protection;
- mempool eviction rules;
- benchmarked transaction cost;
- local adversarial tests that keep spam from being free.

## Current implementation

This first implementation lays down the v0.1-v0.3 foundation:

| Area | Status |
|---|---|
| Rust workspace | Implemented |
| Core account model | Implemented |
| Balances and nonces | Implemented |
| Fee engine | Implemented |
| Ed25519 wallet keys | Implemented |
| Signed transfer transactions | Implemented |
| Local block production | Implemented |
| Binary `.dtx` transaction files | Implemented |
| Binary `.dblk` block files | Implemented |
| CLI inspection | Implemented |
| File-backed local storage | Implemented |
| Mempool policy module | Implemented |
| Local lab report module | Implemented |
| TUI skeleton | Implemented |
| Local P2P | Stubbed for v0.5 |
| Benchmarks | CLI micro-bench scaffold; Criterion files included |

## Quick start

```bash
cargo build --release -p dust-cli
alias dust="./target/release/dust"

dust init
dust wallet new alice
dust wallet new bob
dust faucet alice 1000
dust tx send alice bob 100
dust mine
dust balance alice
dust balance bob
dust chain verify
```

Expected shape:

```text
alice balance: 899 dust
bob balance: 100 dust
chain status: valid
```

The one-dust fee is intentional: the base fee includes the first kilobyte of a normal transfer. Larger transactions pay a marginal size fee.

## Fee model

```text
required_fee = base_fee + marginal_size_fee
paid_fee     = required_fee + priority_fee
```

Default policy:

```toml
[fees]
base_fee = 1
fee_per_kb = 1
included_bytes = 1024
max_priority_fee = 1000
max_tx_size_bytes = 2048
max_memo_bytes = 128
```

A normal transfer should remain cheap. Oversized or spam-like traffic is still priced and bounded.

## CLI

```bash
dust init
dust wallet new alice
dust wallet list
dust wallet show alice
dust faucet alice 1000
dust tx send alice bob 100 --priority-fee 0
dust tx inspect .dustchain/mempool/<hash>.dtx
dust mine
dust chain height
dust chain inspect 1
dust chain verify
dust chain db-stats
dust balance alice
dust fee estimate --memo "hello"
dust mempool list
dust mempool clear
dust inspect block .dustchain/blocks/00000001.dblk
dust inspect tx .dustchain/mempool/<hash>.dtx
dust bench
dust tui
dust lab spam --txs 50000
```

## Binary format

Blocks and transactions are not stored as JSON.

| File | Extension | Magic |
|---|---:|---:|
| Signed transaction | `.dtx` | `DUSTTX` |
| Block | `.dblk` | `DUSTBLK` |

Both formats use fixed magic bytes, protocol version bytes, length-prefixed payloads, and BLAKE3 checksums. The inspector rejects malformed lengths and bad checksums instead of panicking.

## Local lab

The lab module is local-only. It does not target public networks and does not contain offensive third-party attack tooling.

```bash
dust lab spam --txs 50000
dust lab replay
dust lab invalid-tx
dust lab invalid-block
dust lab oversized-block
dust lab fork
```

## Benchmarks

The CLI benchmark command reports measured local values from the current process. Do not copy placeholder numbers into the README. Run:

```bash
dust bench
dust bench --markdown > BENCHMARKS.md
```

## Architecture

```text
dustchain/
├── crates/
│   ├── dust-core      # accounts, state, txs, blocks, validation, fees
│   ├── dust-crypto    # Ed25519 keys, signatures, addresses
│   ├── dust-wire      # custom binary encoding and inspection
│   ├── dust-store     # local file-backed node state
│   ├── dust-mempool   # policy, ordering, eviction
│   ├── dust-node      # node/networking boundary for v0.5
│   ├── dust-tui       # terminal dashboard skeleton
│   ├── dust-lab       # local adversarial simulations
│   └── dust-cli       # `dust` command
├── docs/
├── examples/
├── tests/
└── benches/
```

## Security

`dustchain` is experimental. It has not been audited. It should not be used with real funds, real assets, or public-network security assumptions.

## License

MIT.
