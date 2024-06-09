use criterion::{criterion_group, criterion_main, Criterion};
use db_bench::*;

fn sled_benchmarks(c: &mut Criterion) {
    c.bench_function("SLED DB COLD INITIALIZATION", |b| b.iter(|| sled()));
    println!(
        "SLED DB SIZE AFTER COLD INITIALIZATION: {}",
        measure_sled_size()
    );

    c.bench_function("SLED DB 10 ADDS", |b| b.iter(|| add_to_sled(sled(), 0, 10)));
    println!("SLED DB SIZE AFTER 10 ADDS: {}", measure_sled_size());

    c.bench_function("SLED DB 100 ADDS", |b| b.iter(|| add_to_sled(sled(), 10, 110)));
    println!("SLED DB SIZE AFTER 100 ADDS: {}", measure_sled_size());

    c.bench_function("SLED DB 1000 ADDS", |b| {
        b.iter(|| add_to_sled(sled(), 110, 1110))
    });
    println!("SLED DB SIZE AFTER 1000 ADDS: {}", measure_sled_size());

    c.bench_function("SLED DB 10000 ADDS", |b| {
        b.iter(|| add_to_sled(sled(), 1110, 11110))
    });
    println!("SLED DB SIZE AFTER 10000 ADDS: {}", measure_sled_size());

    c.bench_function("SLED DB 100000 ADDS", |b| {
        b.iter(|| add_to_sled(sled(), 11110, 111110))
    });
    println!("SLED DB SIZE AFTER 100000 ADDS: {}", measure_sled_size());
}

criterion_group!(benches, sled_benchmarks);
criterion_main!(benches);
