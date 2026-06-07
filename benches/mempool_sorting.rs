use criterion::{criterion_group, criterion_main, Criterion};

fn bench_mempool_sorting(c: &mut Criterion) {
    c.bench_function("mempool_sorting", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, bench_mempool_sorting);
criterion_main!(benches);
