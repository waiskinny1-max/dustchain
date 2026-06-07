use dust_core::{Address, FeePolicy, Hash, SignedTransaction, State, Transaction};

#[test]
fn nonce_mismatch_is_rejected() {
    let policy = FeePolicy::default();
    let mut state = State::new();
    state.credit(Address::ZERO, 100);
    let tx = Transaction::transfer("dust-local", Address::ZERO, Address::ZERO, 1, 1, 1, 0, Vec::<u8>::new());
    let signed = SignedTransaction::new(tx, [0u8; 32], [0u8; 64], 180, Hash::ZERO);
    let err = state.apply_transaction(&signed, &policy, |_| true).unwrap_err();
    assert!(err.to_string().contains("nonce mismatch"));
}
