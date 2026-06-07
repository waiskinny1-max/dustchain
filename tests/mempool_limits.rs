use dust_core::{Address, Hash, SignedTransaction, Transaction};
use dust_mempool::{Mempool, MempoolPolicy};

#[test]
fn duplicate_tx_is_rejected() {
    let mut pool = Mempool::new(MempoolPolicy::default());
    let tx = Transaction::transfer("dust-local", Address::ZERO, Address::ZERO, 1, 0, 1, 0, Vec::<u8>::new());
    let signed = SignedTransaction::new(tx, [0u8; 32], [0u8; 64], 180, Hash::digest(b"x"));
    pool.insert(signed.clone()).unwrap();
    assert!(pool.insert(signed).is_err());
}
