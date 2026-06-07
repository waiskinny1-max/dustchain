# Native GUI

`dust-gui` is a native desktop console for the local `dustchain` store.

It is intentionally closer to a traditional wallet client than to a web dashboard:

- persistent left navigation;
- wallet/account table;
- send transaction form;
- local chain and block list;
- mempool table;
- fee estimator;
- binary `.dtx` / `.dblk` inspector;
- local lab runner;
- localnet command reference.

The GUI does not include market data, token price panels, exchange integrations, promotional copy, or production-wallet promises.

## Launch

```bash
cargo run -p dust-gui
```

or through the CLI:

```bash
cargo run -p dust-cli -- gui
```

Use a custom store:

```bash
cargo run -p dust-gui -- --data-dir=.dustchain-node-a
```

## Screens

### Overview

Shows height, mempool count, wallet count, database bytes, latest block, state root, and fast actions.

### Wallet

Creates local development wallets and displays balances/nonces from the state snapshot.

### Send

Signs a transfer, computes deterministic protocol fees, validates against local state, and writes a `.dtx` file to the mempool directory.

### Chain

Lists local `.dblk` files by height and shows state root / transaction root.

### Mempool

Lists pending `.dtx` files and fee details.

### Fees

Shows the active fee policy and estimates minimum required fee for a basic local transfer.

### Inspector

Reads `.dtx` and `.dblk` files, parses magic/version/length/checksum, and renders human-readable structure.

### Lab

Runs safe local simulations only. It never targets third-party nodes.

### Node

Documents the loopback localnet commands. TCP node execution remains in the CLI because it is easier to supervise in a terminal.

## Screenshot policy

The PNG files in `assets/screenshots/` are repository showcase captures. They are designed to communicate the intended GUI layout in GitHub. Before tagging a public release, run the GUI locally and replace these captures with actual runtime screenshots if the interface changed.
