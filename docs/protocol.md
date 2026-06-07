# Protocol

`dustchain` is an account-based local blockchain prototype.

## Account

Each account stores:

```text
address
balance
nonce
```

The nonce is part of transaction validation and prevents replay inside the same chain.

## Transaction

A transfer contains:

```text
version
chain_id
from
to
amount
nonce
max_fee
priority_fee
memo
public_key
signature
```

The signature signs the transaction payload without the signature fields. The public key must hash to the sender address.

## Block

A block contains:

```text
header
transactions
```

The header stores chain ID, height, previous hash, state root, transaction root, timestamp, producer, and local consensus fields.

## Consensus

v0.1 uses local block production. Dev proof-of-work and round-robin validator mode are reserved for later versions.


## v0.4 persistence note

The protocol objects are persisted in an inspectable local store. Blocks remain `.dblk` files and pending transactions remain `.dtx` files. The store now maintains a metadata manifest and a block index so the local chain can be inspected without parsing every file manually.
