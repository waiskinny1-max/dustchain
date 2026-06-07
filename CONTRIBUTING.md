# Contributing

This repo is a protocol-engineering project. Keep changes measured, testable, and non-promotional.

## Before opening a pull request

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Rules

- Do not add investment language.
- Do not fake benchmark numbers.
- Do not add third-party attack functionality.
- Keep lab simulations local-only.
- Keep GUI copy specific and minimal.
- Prefer precise errors over generic failures.

## Good contributions

- smaller binary encoding;
- better validation errors;
- malformed input tests;
- reproducible benchmark fixtures;
- storage abstraction;
- GUI accessibility fixes;
- clearer protocol documentation.
