#!/usr/bin/env bash
set -euo pipefail

# Local-only demo. Run node commands in separate terminals.

cargo run -p dust-cli -- --data-dir .node-a init
cargo run -p dust-cli -- --data-dir .node-b init

cargo run -p dust-cli -- --data-dir .node-a peer add 127.0.0.1:3031
cargo run -p dust-cli -- --data-dir .node-b peer add 127.0.0.1:3030

echo "Terminal A: cargo run -p dust-cli -- --data-dir .node-a node start --port 3030"
echo "Terminal B: cargo run -p dust-cli -- --data-dir .node-b node start --port 3031"
echo "Probe:      cargo run -p dust-cli -- --data-dir .node-a peer probe 127.0.0.1:3031"
