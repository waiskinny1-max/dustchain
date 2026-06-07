# Protocol

`dustchain` is a local-first account-based payment chain experiment.

## Account model

Each account has:

```text
address
balance
nonce
```

The nonce prevents replay of already-applied transactions.

## Transaction model

Each transfer includes:

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
signature
```

A transaction is accepted only if:

1. the signature verifies;
2. the public key maps to the sender address;
3. the chain ID matches local config;
4. the nonce equals the sender account nonce;
5. the amount is greater than zero;
6. the sender can pay amount plus fee;
7. the memo is within policy;
8. the encoded transaction size is within policy;
9. the max fee covers the required protocol fee.

## Block model

A block contains a header and a transaction body. The header commits to:

```text
version
chain_id
height
previous block hash
state root
transaction root
timestamp
producer
difficulty/slot
nonce/round
```

## Local consensus

The current implementation is a development block-production mode suitable for local testing. It is not a mainnet consensus protocol.

## Fee model

```text
fee = base_fee + size_fee + optional_priority_fee
```

Transactions are never free. The minimum fee exists to avoid totally free spam, while compact encoding keeps ordinary transfers cheap.
