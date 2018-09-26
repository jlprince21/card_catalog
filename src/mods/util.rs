extern crate dotenv;
extern crate walkdir;

use std::env;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub struct Settings {
    pub directory_to_scan: String,
    pub pg_connection_string: String
}

pub fn does_file_exist(file_path: &String) -> bool {
    Path::new(file_path).exists()
}

pub fn escape_sql_string(file_path: &String) -> String {
    str::replace(file_path, "'", "''")
}

pub fn get_file_len(file_path: &String) -> i64 {
    use std::fs;
    let metadata = fs::metadata(&file_path).unwrap();
    metadata.len() as i64
}

pub fn get_settings() -> Settings {
    dotenv::dotenv().ok();

    Settings {
        directory_to_scan: env::var("DIRECTORY_TO_SCAN").expect("DIRECTORY_TO_SCAN must be set"),
        pg_connection_string: env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }
}

pub fn is_dir(entry: &DirEntry) -> bool {
    use std::fs;
    let metadata = fs::metadata(entry.path().display().to_string()).unwrap();
    metadata.is_dir()
}

pub fn unescape_sql_string(file_path: &String) -> String {
    str::replace(file_path, "''", "'")
}