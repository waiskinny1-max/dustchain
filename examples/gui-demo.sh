#!/usr/bin/env bash
set -euo pipefail

cargo run -p dust-cli -- --data-dir .dustchain-gui-demo init
cargo run -p dust-cli -- --data-dir .dustchain-gui-demo wallet new alice
cargo run -p dust-cli -- --data-dir .dustchain-gui-demo wallet new bob
cargo run -p dust-cli -- --data-dir .dustchain-gui-demo faucet alice 1000
cargo run -p dust-cli -- --data-dir .dustchain-gui-demo tx send alice bob 100
cargo run -p dust-cli -- --data-dir .dustchain-gui-demo mine
cargo run -p dust-gui -- --data-dir=.dustchain-gui-demo
