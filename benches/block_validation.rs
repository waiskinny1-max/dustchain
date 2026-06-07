use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dust_core::{Chain, FeePolicy};

fn bench_chain_height_read(c: &mut Criterion) {
    let chain = Chain::new("dust-local", FeePolicy::default());
    c.bench_function("chain_height_read", |b| b.iter(|| black_box(chain.height())));
}

criterion_group!(benches, bench_chain_height_read);
criterion_main!(benches);
