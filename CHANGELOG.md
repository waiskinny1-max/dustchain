# Changelog

## v1.0.0-portfolio-release

Full portfolio release candidate.

### Added

- Native desktop GUI crate: `dust-gui`.
- CLI launcher command: `dust gui`.
- GUI screens for overview, wallets, send, chain, mempool, fees, binary inspector, local lab, and localnet notes.
- GUI documentation in `docs/gui.md`.
- Static showcase screenshots in `assets/screenshots/`.
- Full README refresh for v1.0 scope.
- CI dependency installation for Linux GUI builds.

### Preserved

- CLI-first workflow.
- Compact binary `.dtx` and `.dblk` formats.
- Deterministic fee policy.
- Local-only adversarial lab boundary.
- Loopback-first P2P defaults.
- Honest benchmark policy: no fabricated benchmark numbers.

## v0.5.0-localnet

- Local TCP node listener.
- Peer list persistence.
- Peer probing.
- One-shot block fetch.
- One-shot mempool gossip.
- Capped frames and malformed-message handling.

## v0.4.0-storage

- Metadata manifest.
- Block index.
- Atomic file writes.
- Database stats.
- State export.
- Reindex command.

## v0.1.0-v0.3.0-foundation

- Core account model.
- Wallet creation.
- Signed transfer transactions.
- Fee calculation.
- Block production.
- Chain verification.
- Binary transaction and block formats.
- CLI commands.
