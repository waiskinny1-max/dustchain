# Binary format

Blocks and transactions use `dust-wire`, a compact custom binary format.

## Transaction file

Extension: `.dtx`

```text
magic: DUSTTX
version: u8
payload_length: varint
payload: signed transaction
checksum: blake3(frame_without_checksum)
```

## Block file

Extension: `.dblk`

```text
magic: DUSTBLK
version: u8
payload_length: varint
payload: block header + transactions
checksum: blake3(frame_without_checksum)
```

## Safety

The decoder checks lengths, magic bytes, versions, and checksums. Malformed input returns a structured error.
