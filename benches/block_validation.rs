use criterion::{criterion_group, criterion_main, Criterion};

fn bench_block_validation(c: &mut Criterion) {
    c.bench_function("block_validation", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, bench_block_validation);
criterion_main!(benches);
