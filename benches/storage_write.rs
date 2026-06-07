use criterion::{criterion_group, criterion_main, Criterion};

fn bench_storage_write(c: &mut Criterion) {
    c.bench_function("storage_write", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, bench_storage_write);
criterion_main!(benches);
