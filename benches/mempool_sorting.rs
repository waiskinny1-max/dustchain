use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dust_core::{Address, FeePolicy, Hash, SignedTransaction, Transaction};
use dust_mempool::{Mempool, MempoolPolicy};

fn sample_tx(nonce: u64, priority_fee: u64) -> SignedTransaction {
    let tx = Transaction::transfer("dust-local", Address::ZERO, Address::ZERO, 1, nonce, 1, priority_fee, Vec::<u8>::new());
    SignedTransaction::new(tx, [0u8; 32], [0u8; 64], 180, Hash::digest(nonce.to_le_bytes()))
}

fn bench_mempool_sorting(c: &mut Criterion) {
    let fees = FeePolicy::default();
    let mut pool = Mempool::new(MempoolPolicy { max_txs: 2_000, max_txs_per_account: 2_000, max_txs_per_peer: 2_000 });
    for i in 0..1_000 {
        pool.insert(sample_tx(i, i % 17)).unwrap();
    }
    c.bench_function("mempool_ordered_by_fee", |b| b.iter(|| black_box(pool.ordered_by_fee(&fees))));
}

criterion_group!(benches, bench_mempool_sorting);
criterion_main!(benches);
