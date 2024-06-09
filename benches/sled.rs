use criterion::{criterion_group, criterion_main, Criterion};
use db_bench::*;

fn sled_benchmarks(c: &mut Criterion) {
    c.bench_function("initialize", |b| b.iter(|| initialize_sled()));
}

criterion_group!(benches, sled_benchmarks);
criterion_main!(benches);
