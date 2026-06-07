# Benchmark report

No benchmark numbers are committed in this release candidate because they were not generated on verified local hardware.

Generate real results:

```bash
cargo bench
cargo run -p dust-cli -- bench --markdown > BENCHMARKS.md
```

Required metrics before public release:

| Metric | Result |
|---|---:|
| Average tx size | pending local benchmark |
| Minimum transfer fee | pending local benchmark |
| Txs per 1MB block | pending local benchmark |
| Block validation time | pending local benchmark |
| State update time | pending local benchmark |
| Mempool insert time | pending local benchmark |
| DB size per 10k txs | pending local benchmark |
