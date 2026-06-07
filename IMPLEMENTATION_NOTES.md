# Implementation notes

This repository is intended to be built locally with Rust stable.

## Required local checks

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo bench
```

## GUI dependencies on Linux

Native GUI builds may require common desktop/X11 packages. On Ubuntu GitHub runners the CI installs:

```bash
sudo apt-get install -y \
  libgtk-3-dev \
  libx11-dev \
  libxcb1-dev \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libxkbcommon-dev
```

## Benchmark policy

Do not commit invented benchmark numbers. Run the benchmark suite locally and paste the real output into `BENCHMARKS.md` or `docs/benchmarks.md`.

## GUI screenshot policy

The checked-in PNGs under `assets/screenshots/` are showcase captures. Replace them with runtime screenshots after materially changing `dust-gui`.

## Security boundary

The lab and network code are for local protocol testing only. Keep defaults on loopback and do not add third-party scanning or attack behavior.
