# Benchmarks

Benchmark numbers must be generated from the local machine. Do not fake benchmark tables.

CLI benchmark commands:

```bash
dust bench
dust bench --markdown > BENCHMARKS.md
```

Storage-related checks added in v0.4:

```bash
dust chain db-stats --verbose
dust chain reindex
```

The storage stats are not performance benchmarks. They are footprint measurements showing the byte cost of blocks, pending transactions, wallets, snapshots, and metadata.

Recommended release table fields for v0.7:

| Metric | Source |
|---|---|
| Average tx size | `dust bench --markdown` |
| Minimum transfer fee | `dust bench --markdown` |
| Txs per 1MB block | `dust bench --markdown` |
| Block validation time | Criterion benchmark |
| State update time | Criterion benchmark |
| Mempool insert time | Criterion benchmark |
| DB size per 10k txs | Storage benchmark |
