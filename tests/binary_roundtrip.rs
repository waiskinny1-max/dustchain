use dust_core::{Address, Hash, SignedTransaction, Transaction};
use dust_wire::{decode_signed_transaction_payload, signed_tx_payload};

#[test]
fn signed_tx_binary_roundtrip() {
    let tx = Transaction::transfer("dust-local", Address::ZERO, Address::ZERO, 1, 0, 1, 0, Vec::<u8>::new());
    let mut signed = SignedTransaction::new(tx, [7u8; 32], [9u8; 64], 0, Hash::ZERO);
    let payload = signed_tx_payload(&signed);
    signed.encoded_size = payload.len();
    signed.hash = Hash::digest(&payload);

    let decoded = decode_signed_transaction_payload(&payload).unwrap();
    assert_eq!(decoded.tx.amount, 1);
    assert_eq!(decoded.public_key, [7u8; 32]);
}
