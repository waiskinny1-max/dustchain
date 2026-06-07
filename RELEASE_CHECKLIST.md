# v1.0 release checklist

Before tagging `v1.0.0-portfolio-release`:

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo bench`
- [ ] `cargo run -p dust-cli -- init`
- [ ] `cargo run -p dust-cli -- wallet new alice`
- [ ] `cargo run -p dust-cli -- wallet new bob`
- [ ] `cargo run -p dust-cli -- faucet alice 1000`
- [ ] `cargo run -p dust-cli -- tx send alice bob 100`
- [ ] `cargo run -p dust-cli -- mine`
- [ ] `cargo run -p dust-cli -- chain verify`
- [ ] `cargo run -p dust-cli -- inspect block ./.dustchain/blocks/00000001.dblk`
- [ ] `cargo run -p dust-gui`
- [ ] Refresh GUI screenshots from the running app if the UI changed.
- [ ] Replace pending benchmark table with real numbers.
- [ ] Confirm `SECURITY.md` still says the project is experimental and local-first.
