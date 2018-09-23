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
    directory_to_scan: String,
    pg_connection_string: String
}

use self::models::{NewListing, Listing};

fn main() {
    let settings: Settings = get_settings();

    let connection = establish_connection(&settings.pg_connection_string);

    // start_hashing(&settings.directory_to_scan, &connection);
    // delete_missing_listings(&connection);
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

fn delete_missing_listings(conn: &PgConnection) {
    use schema::listings::dsl::*;

    match find_missing(&conn) {
        None => println!("none missing"),
        Some(x) =>
            {
                println!("some missing");
                for y in x {
                    println!("removing listing for missing file: {}", y.file_path);

                    // 18-09-23 TODO: One day, may want to mark listings as deleted instead of removing them
                    diesel::delete(listings.filter(file_path.eq(y.file_path)))
                        .execute(conn)
                        .expect("Error deleting listing");
                }
            }
    }
}

fn does_file_exist(file_path: &String) -> bool {
    Path::new(file_path).exists()
}

fn escape_sql_string(file_path: &String) -> String {
    str::replace(file_path, "'", "''")
}

fn establish_connection(connection: &String) -> PgConnection {
    PgConnection::establish(&connection).expect(&format!("Error connecting to {}", connection))
}

fn find_missing(conn: &PgConnection) -> Option<Vec<models::Listing>> {
    use schema::listings::dsl::*;

    let results = listings
        .load::<Listing>(conn)
        .expect("Error loading listings");

    let mut missing: Vec<models::Listing> = Vec::new();

    for listing in results {
        if does_file_exist(&unescape_sql_string(&listing.file_path)) == false {
            missing.push(listing);
        }
    }

    match missing.len() {
        0 => None,
        _ => Some(missing)
    }
}

fn get_file_len(file_path: &String) -> i64 {
    let metadata = std::fs::metadata(&file_path).unwrap();
    metadata.len() as i64
}

fn get_settings() -> Settings {
    dotenv().ok();

    Settings {
        directory_to_scan: env::var("DIRECTORY_TO_SCAN").expect("DIRECTORY_TO_SCAN must be set"),
        pg_connection_string: env::var("DATABASE_URL").expect("DATABASE_URL must be set")
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

            let is_hashed: bool = is_file_hashed(&escape_sql_string(&file_path), &conn);

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

fn unescape_sql_string(file_path: &String) -> String {
    str::replace(file_path, "''", "'")
}