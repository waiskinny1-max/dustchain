use criterion::{criterion_group, criterion_main, Criterion};

fn bench_tx_encoding(c: &mut Criterion) {
    c.bench_function("tx_encoding", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, bench_tx_encoding);
criterion_main!(benches);
