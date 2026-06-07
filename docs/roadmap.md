# Roadmap

## Completed release line

| Version | Focus | Outcome |
|---|---|---|
| v0.1 | Core local chain | wallets, signed transactions, balances, mining, chain verification |
| v0.2 | Fee engine | deterministic base/size/priority fee model |
| v0.3 | Binary format | `.dtx`, `.dblk`, checksums, inspectors |
| v0.4 | Storage | persistent local store, metadata, index, state export |
| v0.5 | Localnet | loopback TCP nodes, peer status, block fetch, mempool gossip |
| v0.6 | TUI | terminal dashboard snapshot |
| v0.7 | Benchmarks | Criterion scaffolding and CLI report surface |
| v0.8 | Local lab | spam/replay/invalid-input simulations bounded to local testing |
| v0.9 | Documentation | protocol, fees, binary format, networking, threat model |
| v1.0 | Portfolio release | full workspace, GUI, screenshots, examples, CI, release docs |

## v1.1 candidates

- Replace file-backed storage with an embedded database backend behind a trait.
- Add a live TUI event loop instead of a snapshot renderer.
- Add GUI runtime screenshot capture script.
- Add round-robin validator mode for easier local multi-node demos.
- Add property tests for malformed binary frames and block roundtrips.
- Add reproducible benchmark fixture generator.

## Non-goals

- Public cryptocurrency launch.
- Token sale or exchange listing.
- Smart-contract VM.
- Mainnet networking.
- Attack tooling against third-party systems.
