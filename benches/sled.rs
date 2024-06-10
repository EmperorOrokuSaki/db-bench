use std::{fs, io, path::Path};

use criterion::{criterion_group, criterion_main, Criterion};
use db_bench::*;
use sled::Config;

fn generate_bytes(length: usize, salt: u32) -> Vec<u8> {
    salt.to_be_bytes()
        .into_iter()
        .cycle()
        .take(length)
        .collect()
}

fn print_size(db_path: &str) {
    match get_dir_size(db_path) {
        Ok(size) => {
            let formatted_size = format_size(size);
            println!("Size of database at '{}': {}", db_path, formatted_size);
        }
        Err(e) => eprintln!("Failed to get size of '{}': {}", db_path, e),
    }
}

fn get_dir_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let mut size = 0;
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            size += get_dir_size(entry.path())?;
        } else {
            size += metadata.len();
        }
    }
    Ok(size)
}

fn format_size(bytes: u64) -> String {
    let sizes = ["Bytes", "KB", "MB", "GB", "TB"];
    let factor = 1024.0;
    let mut size = bytes as f64;

    let mut i = 0;
    while size >= factor && i < sizes.len() - 1 {
        size /= factor;
        i += 1;
    }

    format!("{:.2} {}", size, sizes[i])
}
fn sled_bulk_insertions(c: &mut Criterion) {
    let base : u64 = 2;
    for index in (16 as u32..=54).step_by(2) {
        let length = base.pow(index) as usize;
        let db_path = format!("bulk_insertions_{}", length);
        let db = Config::new()
            .path(db_path.clone())
            .flush_every_ms(None)
            .open()
            .unwrap();

        c.bench_function(
            &format!("Inserting {} bytes with value {}", length, index),
            |b| {
                b.iter(|| {
                    db.insert(index.to_be_bytes(), generate_bytes(length.clone(), index))
                        .unwrap();
                });
            },
        );

        print_size(&db_path);
        fs::remove_dir_all(&db_path).unwrap();
    }
}

criterion_group!(benches, sled_bulk_insertions);
criterion_main!(benches);
