# Local Networking

`dustchain` v0.5 adds a local-first TCP control plane for running multiple development nodes on one machine or inside a controlled LAN lab.

This networking layer is intentionally narrow. It is not a public mainnet protocol, not a DHT, and not an attack tool. The default bind host is `127.0.0.1`, and non-loopback binding is refused unless explicitly enabled.

## Commands

```bash
dust node start --port 3030
dust node start --port 3031 --peer 127.0.0.1:3030
dust peer add 127.0.0.1:3031
dust peer list
dust peer probe 127.0.0.1:3031
dust peer fetch-block 127.0.0.1:3031 1
dust peer gossip-mempool 127.0.0.1:3031
```

## Peer Messages

The v0.5 wire control protocol is line-delimited text wrapped around existing binary block and transaction files. This keeps the node easy to inspect while preserving `.dblk` and `.dtx` as the canonical storage formats.

Supported local messages:

```text
STATUS
HELLO <chain_id> <height> <tip_hash>
GET_BLOCK <height>
GET_MEMPOOL
SUBMIT_TX <hex_encoded_dtx_file>
PING
```

Responses:

```text
STATUS <chain_id> <height> <tip_hash> <mempool_txs>
BLOCK <height> <hex_encoded_dblk_file>
MEMPOOL <count> <comma_separated_hashes>
ACCEPTED_TX <tx_hash> file=<path>
NOT_FOUND block <height>
PONG
ERROR <reason>
```

## Safety Properties

- Default bind address is loopback only.
- Non-loopback hosts require `--allow-non-loopback`.
- Frames are size-capped.
- Unknown peer messages return structured errors.
- Peers are disconnected after repeated malformed messages.
- Fetched blocks are written to `.dustchain/synced/` and are not appended automatically.
- Transaction gossip only submits existing local `.dtx` mempool files.

## Current Limitation

v0.5 provides local peer status, block fetch, and mempool gossip. It does not automatically perform chain reorganization, remote block import, NAT traversal, peer discovery, or public-network sync.
