#!/usr/bin/env bash
set -euo pipefail

cargo run -p dust-cli -- init
cargo run -p dust-cli -- wallet new alice
cargo run -p dust-cli -- wallet new bob
cargo run -p dust-cli -- faucet alice 1000
cargo run -p dust-cli -- tx send alice bob 100
cargo run -p dust-cli -- mine
cargo run -p dust-cli -- balance alice
cargo run -p dust-cli -- balance bob
cargo run -p dust-cli -- chain verify
