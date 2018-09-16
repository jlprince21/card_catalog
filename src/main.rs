extern crate config;
extern crate twox_hash;
extern crate walkdir;

#[macro_use]
extern crate mysql;

// MariaDB
use mysql as my;

use walkdir::{DirEntry, WalkDir};

// file hashing
use twox_hash::XxHash;
use std::hash::Hasher;
use std::io::BufReader;

// file loading/metadata
use std::fs::File;
use std::fs::Metadata;
use std::io::prelude::*;
use std::path::Path;

// derive line enables easy printing of struct
#[derive(Debug)]
struct Listing {
    file_name: String
}

fn main() {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("settings"))
        .expect("Config file missing!");

    let sql_user = get_setting(&settings, "SQLUser".to_string());
    let sql_password = get_setting(&settings, "SQLPassword".to_string());
    let sql_server = get_setting(&settings, "SQLServer".to_string());
    let sql_port = get_setting(&settings, "SQLPort".to_string());
    let sql_database = get_setting(&settings, "SQLDatabase".to_string());

    let connection_string = format!("mysql://{}:{}@{}:{}/{}", sql_user, sql_password, sql_server, sql_port, sql_database);
    println!("{}", connection_string);

    let pool = my::Pool::new(connection_string).unwrap();

    start_hashing(&"/path/to/file".to_string(), &pool);

    println!("{}", hash_file(&"/path/to/file".to_string()));

    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("settings"))
        .expect("Config file missing!");

    let sql_user = get_setting(&settings, "SQLUser".to_string());
    let sql_password = get_setting(&settings, "SQLPassword".to_string());
    let sql_server = get_setting(&settings, "SQLServer".to_string());
    let sql_port = get_setting(&settings, "SQLPort".to_string());
    let sql_database = get_setting(&settings, "SQLDatabase".to_string());

    let connection_string = format!("mysql://{}:{}@{}:{}/{}", sql_user, sql_password, sql_server, sql_port, sql_database);
    println!("{}", connection_string);

    let pool = my::Pool::new(connection_string).unwrap();

    let selected_listings: Vec<Listing> =
    pool.prep_exec("SELECT FileName from Listings limit 10", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Listing>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Listing`
        result.map(|x| x.unwrap()).map(|row| {
            // Note that from_row will panic if you don't follow your schema
            let FileName = my::from_row(row);
            Listing {
                file_name: FileName,
            }
        }).collect() // Collect Listings so now `QueryResult` is mapped to `Vec<Listing>`
    }).unwrap(); // Unwrap `Vec<Listing>`

    println!("{:?}", selected_listings);
}

fn get_file_len(file_path: &String) -> u64 {
    let metadata = std::fs::metadata(&file_path).unwrap();
    metadata.len()
}

fn get_setting(settings: &config::Config, key: String) -> String {
    format!("{}", settings.get_str(&key).unwrap())
}

fn hash_file(file_name: &String) -> String {
    let path = Path::new(&file_name);
    if !path.exists() {
        return String::from("Not Found!").into();
    }

    let mut file_hasher = XxHash::default();

    // via https://stackoverflow.com/q/37079342
    const CAP: usize = 1024 * 128; // 18-09-13 Increasing buffer size doesn't seem to improve performance.
    let mut file = File::open(&file_name).expect("Unable to open file");
    let mut reader = BufReader::with_capacity(CAP, file);

    loop {
        let length = {
            let buffer = reader.fill_buf().expect("Read error");
            file_hasher.write(buffer);
            buffer.len()
        };
        if length == 0 { break; }
        reader.consume(length);
    }

    let hash = file_hasher.finish();
    format!("{:x}", hash)
}

fn is_dir(entry: &DirEntry) -> bool {
    let metadata = std::fs::metadata(entry.path().display().to_string()).unwrap();
    metadata.is_dir()
}

fn start_hashing(root_directory: &String, pool: &my::Pool) {
    let walker = WalkDir::new(root_directory).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if is_dir(&entry) {
            // do nothing
        } else {
            let file_path = entry.path().display().to_string();
            // println!("{} {:?} {} {}", get_file_len(&file_path), entry.file_name(), hash_file(&file_path), &file_path);

            // TODO 18-09-16 This SQL is pretty bad. Need to use params!, error checkign, and clean this up in general.
            let command = format!(r"INSERT INTO `Listings`
                    (`FileName`, `FilePath`, `Checksum`, `FileSize`)
                    VALUES
                    ('{}', '{}', '{}', {})",
                    entry.file_name().to_str().unwrap(),
                    entry.path().display().to_string(),
                    hash_file(&file_path),
                    get_file_len(&file_path));
            println!("command: {}", command);
            pool.prep_exec(command, ());
        }
    }
}