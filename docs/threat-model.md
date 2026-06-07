# Threat model

`dustchain` is a local protocol engineering project. It is not production secure.

## In scope

- malformed local files;
- invalid signatures;
- replay attempts;
- oversized transactions;
- oversized blocks;
- local mempool spam simulation;
- duplicate transaction rejection.

## Out of scope

- real funds;
- public network consensus;
- validator economics;
- custody;
- denial-of-service testing against third-party systems;
- investment use.

## Security posture

The project should be treated as educational and experimental.
