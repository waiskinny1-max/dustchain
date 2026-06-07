# Security

`dustchain` is an experimental local-first blockchain implementation.

Do not use it for real funds, real assets, or production consensus.

## Supported security boundary

Supported:

- local testing;
- malformed local file handling;
- local mempool spam simulation;
- local replay simulation;
- local invalid transaction simulation;
- local invalid block simulation.

Not supported:

- public-network deployment;
- third-party target testing;
- real financial use;
- wallet custody;
- production validator operation.

## Reporting

Open an issue with:

- affected version;
- reproduction steps;
- input file or command;
- expected behavior;
- observed behavior.

Do not include exploit instructions against third-party networks.
