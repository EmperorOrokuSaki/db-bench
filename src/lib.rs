use std::fs;

use walkdir::WalkDir;

static SLED_PATH: &str = "sled";
static ROCKS_DB_PATH: &str = "rocksdb";

pub fn initialize_sled() {
    sled::open(SLED_PATH).unwrap();
}

pub fn measure_sled_size() -> u64 {
    calculate_directory_size(SLED_PATH)
}

pub fn measure_rocksdb_size() -> u64 {
    calculate_directory_size(ROCKS_DB_PATH)
}

fn calculate_directory_size(path: &str) -> u64 {
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

    total_size
}
