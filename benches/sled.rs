use criterion::{criterion_group, criterion_main, Criterion};
use db_bench::*;

fn sled_benchmarks(c: &mut Criterion) {
    c.bench_function("SLED DB COLD INITIALIZATION", |b| {
        b.iter(|| initialize_sled())
    });
    println!(
        "SLED DB SIZE AFTER COLD INITIALIZATION: {}",
        measure_sled_size()
    );
}

criterion_group!(benches, sled_benchmarks);
criterion_main!(benches);
