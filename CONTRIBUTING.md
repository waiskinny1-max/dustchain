# Contributing

Keep the project boring in the right places.

Required before a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Project rules:

- no token hype;
- no fake benchmark numbers;
- no production-security claims;
- no JSON block or transaction format;
- no public-network adversarial tooling;
- errors must explain the exact rejection reason.
