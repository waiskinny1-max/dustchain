use criterion::{criterion_group, criterion_main, Criterion};
use dust_core::FeePolicy;

fn bench_fee(c: &mut Criterion) {
    let policy = FeePolicy::default();
    c.bench_function("required_fee", |b| b.iter(|| policy.required_fee(184)));
}

criterion_group!(benches, bench_fee);
criterion_main!(benches);
