#!/usr/bin/env bash
set -euo pipefail
cargo run -p dust-cli -- lab spam --txs 50000
