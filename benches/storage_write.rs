use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dust_store::DustStore;

fn bench_storage_init(c: &mut Criterion) {
    c.bench_function("storage_init", |b| {
        b.iter(|| {
            let dir = tempfile::tempdir().unwrap();
            let store = DustStore::open(dir.path().join("chain"));
            store.init().unwrap();
            black_box(store.db_stats().unwrap())
        })
    });
}

criterion_group!(benches, bench_storage_init);
criterion_main!(benches);
