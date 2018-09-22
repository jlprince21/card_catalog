extern crate config;
extern crate twox_hash;
extern crate walkdir;

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

// start diesel

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// end diesel

struct Settings {
    sql_user: String,
    sql_password: String,
    sql_server: String,
    sql_port: String,
    sql_database: String,
}

use self::models::{NewListing, Listing};

fn main() {
    // let settings: Settings = get_settings();

    let connection = establish_connection();

    start_hashing(&"/path/to/file".to_string(), &connection);
}

pub fn create_listing(conn: &PgConnection, checksum: &str, file_name: &str, file_path: &str, file_size: &i64) -> Listing {
    use schema::listings;

    let new_listing = NewListing {
        checksum: checksum,
        file_name: file_name,
        file_path: file_path,
        file_size: file_size,
    };

    diesel::insert_into(listings::table)
        .values(&new_listing)
        .get_result(conn)
        .expect("Error saving new listing")
}

fn escape_sql_string(file_path: &String) -> String {
    str::replace(file_path, "'", "''")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn get_file_len(file_path: &String) -> i64 {
    let metadata = std::fs::metadata(&file_path).unwrap();
    metadata.len() as i64
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
    let file = File::open(&file_name).expect("Unable to open file");
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

fn is_file_hashed(file_path_to_check: &String, conn: &PgConnection) -> bool {
    // TODO 18-09-17 Query for checksum being empty/null instead of a simple count
    use self::schema::listings::dsl::*;

    let results = listings
        .filter(file_path.eq(file_path_to_check))
        .limit(1)
        .load::<Listing>(conn)
        .expect("Error loading posts");

    if results.len() >= 1 {
        return true
    } else {
        return false
    }
}

fn start_hashing(root_directory: &String, conn: &PgConnection) {
    let walker = WalkDir::new(root_directory).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if is_dir(&entry) {
            // do nothing
        } else {
            let file_path = entry.path().display().to_string();

            let is_hashed: bool = is_file_hashed(&file_path, &conn);

            if is_hashed == true {
                println!("skipping hash for {}", &file_path);
            } else {
                // TODO 18-09-22 Need to research diesel error checking and see if strings can be cleaned up
                println!("hashing: {}", &file_path);
                create_listing(&conn,
                            &hash_file(&file_path),
                            &escape_sql_string(&entry.file_name().to_str().unwrap().to_string()),
                            &escape_sql_string(&entry.path().display().to_string()),
                            &get_file_len(&file_path));
            }
        }
    }
}