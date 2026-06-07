# Fees

Low fees are handled as a protocol policy, not as a mining claim.

## Formula

```text
required_fee = base_fee + marginal_size_fee
paid_fee     = required_fee + priority_fee
```

The default policy includes the first 1024 bytes inside the base fee. This keeps a normal transfer at the minimum fee while still charging marginal size fees for larger transactions.

## Defaults

```toml
base_fee = 1
fee_per_kb = 1
included_bytes = 1024
max_priority_fee = 1000
max_tx_size_bytes = 2048
max_memo_bytes = 128
```

## Rejection rules

A transaction is rejected if:

- the encoded transaction exceeds `max_tx_size_bytes`;
- the memo exceeds `max_memo_bytes`;
- the priority fee exceeds `max_priority_fee`;
- `max_fee` is lower than the required fee;
- the sender balance cannot cover amount plus paid fee.
