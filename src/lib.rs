use std::fs;

use walkdir::WalkDir;

static SLED_PATH: &str = "sled";
static ROCKS_DB_PATH: &str = "rocksdb";

pub fn initialize_sled() {
    sled::open(SLED_PATH).unwrap();
}

pub fn add_to_sled() {}

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
