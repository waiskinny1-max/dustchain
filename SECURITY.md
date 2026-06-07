# Security

`dustchain` is an experimental local-first blockchain implementation. It is not audited and is not production-ready.

## Do not use for

- real funds;
- public financial activity;
- mainnet consensus;
- custody;
- exchange integration;
- third-party network testing;
- offensive security operations.

## Local lab boundary

The lab commands simulate invalid, replayed, oversized, spam-like, and fork-like inputs against local test logic only. They must not be extended into tooling for scanning, flooding, exploiting, or disrupting third-party systems.

## Wallet material

Wallet files are plaintext local development files. They are not suitable for production custody.

## Network defaults

Node commands are loopback-first. Non-loopback behavior should remain explicit and documented.

## Reporting issues

Open a GitHub issue with:

- affected command;
- operating system;
- steps to reproduce;
- expected behavior;
- actual behavior;
- relevant logs.

Do not include private keys or sensitive wallet files.
