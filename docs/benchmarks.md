# Benchmarks

Benchmarks are part of the low-fee argument. They must be generated, not invented.

## Commands

```bash
cargo bench
cargo run -p dust-cli -- bench
cargo run -p dust-cli -- bench --markdown
```

## Required metrics

- average transaction size;
- minimum fee;
- average fee;
- transactions per 1MB block;
- block validation time;
- state update time;
- mempool insertion time;
- storage cost per transaction batch.

## Reporting rule

Do not commit synthetic benchmark numbers. If results are not produced by a local run, mark them as pending.
