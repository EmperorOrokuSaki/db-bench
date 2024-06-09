use std::fs;

use sled::Db;
use walkdir::WalkDir;

static SLED_PATH: &str = "sled";
static ROCKS_DB_PATH: &str = "rocksdb";

pub fn sled() -> Db {
    sled::open(SLED_PATH).unwrap()
}

pub fn add_to_sled(db: Db, start: u64, end: u64) {
    for i in start..end {
        // Convert u64 to byte array using little endian format
        let key = i.to_le_bytes();
        let value = i.to_le_bytes();

        // Insert the key-value pair into the sled database
        db.insert(&key, &value).expect("Failed to insert into sled");
    }
}

pub fn measure_sled_size() -> String {
    calculate_directory_size(SLED_PATH)
}

pub fn measure_rocksdb_size() -> String {
    calculate_directory_size(ROCKS_DB_PATH)
}

fn calculate_directory_size(path: &str) -> String {
    let mut total_size = 0;

    // Traverse the directory and its subdirectories
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let metadata = fs::metadata(entry.path()).unwrap();

        // Add file size to total
        if metadata.is_file() {
            total_size += metadata.len();
        }
    }

    let units = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut size = total_size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, units[unit])
}
