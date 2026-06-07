# Roadmap

## v0.1 — Core Local Chain

Status: implemented foundation.

- wallet creation;
- signed transfers;
- faucet for local testing;
- block creation;
- state transitions;
- chain verification.

## v0.2 — Fee Engine

Status: implemented foundation.

- deterministic required fee;
- base fee;
- size fee;
- optional priority fee;
- fee estimate command;
- transaction rejection for insufficient max fee.

## v0.3 — Binary Format

Status: implemented foundation.

- `.dtx` transaction files;
- `.dblk` block files;
- custom binary encoder/decoder;
- checksums;
- inspector commands.

## v0.4 — Persistent Storage

Status: current release.

- file-backed persistent store;
- atomic writes;
- persisted state snapshot;
- persisted genesis snapshot;
- block files;
- pending transaction files;
- metadata manifest;
- block index;
- database statistics;
- state export;
- reindex command.

## v0.5 — Local P2P Network

Next target.

- TCP listener;
- peer config;
- handshake frame;
- transaction gossip;
- block gossip;
- height sync;
- malformed frame rejection;
- local two-node demo.

## v0.6 — Terminal UI

- live Ratatui dashboard;
- blocks screen;
- mempool screen;
- fees screen;
- peers screen;
- logs screen.

## v0.7 — Benchmarks

- real Criterion benchmarks;
- generated markdown report;
- README benchmark table from real output only.

## v0.8 — Local Adversarial Lab

- spam simulation;
- replay simulation;
- invalid transaction simulation;
- invalid block simulation;
- oversized block simulation;
- fork simulation.

## v1.0 — Portfolio Release

- tests pass;
- screenshots added;
- localnet demo works;
- benchmark report committed;
- threat model complete;
- release tag created.
