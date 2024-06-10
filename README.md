# RocksDB vs Sled Benchmark Comparison

This project is dedicated to comparing the performance of two prominent embedded databases, RocksDB and Sled, through a series of benchmarks. This comparison aims to provide insights into the performance characteristics of each database under various conditions.

## Overview

The benchmarks focus on testing the insertion capabilities and storage efficiency of RocksDB and Sled. The tests involve inserting large volumes of data and measuring the performance in terms of insertion time and disk space utilization.

## Benchmarks

### Insertion Performance

- **Data Sizes**: Insertions vary by the size of the data, starting from 2^16 bytes to 2^54 bytes, incrementing by powers of two.
- **Metrics Collected**:
  - Insertion time for each data size.
  - Disk space used by the database after each insertion.

### Storage Efficiency

- After each insertion, the size of the database directory is calculated and logged, providing insight into the storage efficiency of each database.

## Getting Started

### Tools

- Rust toolchain (latest stable release recommended).
- Criterion for benchmarking (included in `Cargo.toml`).
- Sled and RocksDB libraries (included in `Cargo.toml`).

### Running the Benchmarks

```
cargo bench
cargo bench --bench sled
cargo bench --bench rocksdb
```