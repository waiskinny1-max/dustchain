#!/usr/bin/env bash
set -euo pipefail
cargo run -p dust-cli -- inspect block .dustchain/blocks/00000001.dblk
