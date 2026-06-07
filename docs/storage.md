# Storage

`dustchain` v0.4 uses an inspectable file-backed store. This is deliberate: the project is still a protocol engineering portfolio project, so the binary block and transaction files should remain easy to inspect from the terminal.

## Layout

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

## Atomic writes

v0.4 writes critical files through temporary files and renames them into place. This reduces the risk of half-written files during local testing.

Covered file types:

- config;
- wallets;
- genesis snapshot;
- state snapshot;
- pending transaction files;
- block files;
- metadata manifest;
- block index.

## Manifest

The manifest records the current storage view:

```text
schema_version=1
chain_id=dust-local
height=1
tip_hash=<hash>
state_root=<hash>
blocks=1
updated_at_unix=<timestamp>
```

## Block index

The block index is tab-separated:

```text
height  hash  previous  txs  bytes  file
```

Regenerate it with:

```bash
dust chain reindex
```

## Statistics

```bash
dust chain db-stats --verbose
```

Reports total bytes and per-area byte counts for blocks, mempool, wallets, state, and metadata.

## Current limitation

The store is not yet an embedded database. That is acceptable for this stage because v0.4 emphasizes persistence, crash-tolerant writes, and inspectability. A later release can swap the backend while keeping the public store API stable.
