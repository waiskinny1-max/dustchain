use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dust_core::{Address, Hash, SignedTransaction, Transaction};
use dust_wire::signed_tx_payload;

fn bench_tx_encoding(c: &mut Criterion) {
    let tx = Transaction::transfer("dust-local", Address::ZERO, Address::ZERO, 1, 0, 1, 0, Vec::<u8>::new());
    let signed = SignedTransaction::new(tx, [7u8; 32], [9u8; 64], 180, Hash::ZERO);
    c.bench_function("signed_tx_payload", |b| b.iter(|| black_box(signed_tx_payload(&signed))));
}

criterion_group!(benches, bench_tx_encoding);
criterion_main!(benches);
