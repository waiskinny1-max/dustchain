# Changelog

## v0.4.0-storage

Storage hardening release.

Added:

- persistent store metadata manifest at `.dustchain/metadata/manifest.txt`;
- block index at `.dustchain/metadata/block-index.tsv`;
- atomic writes for config, wallets, state snapshots, pending transactions, and blocks;
- `dust chain db-stats --verbose`;
- `dust chain reindex`;
- `dust chain export-state`;
- explicit storage layout documentation;
- storage persistence regression test.

Changed:

- `dust init` now creates metadata files;
- `dust mine` updates the metadata manifest and block index;
- database statistics now report block, mempool, wallet, state, and metadata byte counts separately.

Notes:

- The storage engine is still intentionally file-backed and inspectable. A future embedded database can be added behind the same store API.

## v0.1.0-v0.3.0

Initial protocol engineering foundation:

- account model;
- signed transactions;
- fee model;
- block creation;
- chain verification;
- binary transaction and block files;
- CLI inspection commands;
- docs, tests, examples, and CI skeleton.
