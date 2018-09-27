extern crate twox_hash;
extern crate walkdir;

use self::walkdir::{DirEntry, WalkDir};

use diesel;
use diesel::prelude::*;

// file hashing
use self::twox_hash::XxHash;
use std::hash::Hasher;
use std::io::BufReader;

// file loading/metadata
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use mods::util as Util;
use mods::models as Models;
use mods::sql as Sql;

use Models::{NewListing, Listing};

pub fn delete_missing_listings(conn: &PgConnection) {
    use Schema::listings::dsl::*;

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

pub fn find_missing(conn: &PgConnection) -> Option<Vec<Models::Listing>> {
    use mods::schema::listings::dsl::*;

    let results = listings
        .load::<Listing>(conn)
        .expect("Error loading listings");

    let mut missing: Vec<Models::Listing> = Vec::new();

    for listing in results {
        if Util::does_file_exist(&Util::unescape_sql_string(&listing.file_path)) == false {
            missing.push(listing);
        }
    }

    match missing.len() {
        0 => None,
        _ => Some(missing)
    }
}

pub fn hash_file(file_name: &String) -> String {
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

pub fn is_file_hashed(file_path_to_check: &String, conn: &PgConnection) -> bool {
    // TODO 18-09-17 Query for checksum being empty/null instead of a simple count
    use Schema::listings::dsl::*;

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

pub fn start_hashing(root_directory: &String, conn: &PgConnection) {
    let walker = WalkDir::new(root_directory).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if Util::is_dir(&entry) {
            // do nothing
        } else {
            let file_path = entry.path().display().to_string();

            let is_hashed: bool = is_file_hashed(&Util::escape_sql_string(&file_path), &conn);

            if is_hashed == true {
                println!("skipping hash for {}", &file_path);
            } else {
                // TODO 18-09-22 Need to research diesel error checking and see if strings can be cleaned up
                println!("hashing: {}", &file_path);
                Sql::create_listing(&conn,
                            &hash_file(&file_path),
                            &Util::escape_sql_string(&entry.file_name().to_str().unwrap().to_string()),
                            &Util::escape_sql_string(&entry.path().display().to_string()),
                            &Util::get_file_len(&file_path));
            }
        }
    }
}