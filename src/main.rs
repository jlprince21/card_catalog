extern crate config;
extern crate twox_hash;
extern crate walkdir;

#[macro_use]
extern crate mysql;

// MariaDB
use mysql as my;
use mysql::from_row;

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

struct Settings {
    sql_user: String,
    sql_password: String,
    sql_server: String,
    sql_port: String,
    sql_database: String,
}

fn main() {
    let settings: Settings = get_settings();

    let connection_string = format!("mysql://{}:{}@{}:{}/{}", settings.sql_user, settings.sql_password, settings.sql_server, settings.sql_port, settings.sql_database);
    let pool = my::Pool::new(connection_string).unwrap();

    // start_hashing(&"/path/to/file".to_string(), &pool);
    // print_listings(&pool);
}

fn escape_sql_string(file_path: &String) -> String {
    str::replace(file_path, "'", "''")
}

fn get_file_len(file_path: &String) -> u64 {
    let metadata = std::fs::metadata(&file_path).unwrap();
    metadata.len()
}

fn get_settings() -> Settings {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("settings"))
        .expect("Config file missing!");

    Settings {
        sql_user: get_setting(&settings, "SQLUser".to_string()),
        sql_password: get_setting(&settings, "SQLPassword".to_string()),
        sql_database: get_setting(&settings, "SQLDatabase".to_string()),
        sql_port: get_setting(&settings, "SQLPort".to_string()),
        sql_server: get_setting(&settings, "SQLServer".to_string())
    }
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

fn is_file_hashed(file_path: &String, pool: &my::Pool) -> bool {
    // TODO 18-09-17 Query for checksum being empty/null instead of a simple count
    let query = format!(r"SELECT count(1) from `Listings` where FilePath = '{}'", escape_sql_string(&file_path));

    for row in pool.prep_exec(query, ()).unwrap() {
        let a: u32 = from_row(row.unwrap());

        if a == 0 {
            return false
        } else {
            return true
        }
    }

    false
}

fn print_listings(pool: &my::Pool) {
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

fn start_hashing(root_directory: &String, pool: &my::Pool) {
    let walker = WalkDir::new(root_directory).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if is_dir(&entry) {
            // do nothing
        } else {
            let file_path = entry.path().display().to_string();

            let is_hashed: bool = is_file_hashed(&file_path, &pool);

            if is_hashed == true {
                println!("skipping hash for {}", &file_path);
            } else {
                // TODO 18-09-16 This SQL is pretty bad. Need to use params!, error checking, and general cleanup.
                let command = format!(r"INSERT INTO `Listings`
                        (`FileName`, `FilePath`, `Checksum`, `FileSize`)
                        VALUES
                        ('{}', '{}', '{}', {})",
                        escape_sql_string(&entry.file_name().to_str().unwrap().to_string()),
                        escape_sql_string(&entry.path().display().to_string()),
                        hash_file(&file_path),
                        get_file_len(&file_path));
                println!("hashing: {}", command);
                pool.prep_exec(command, ());
            }
        }
    }
}