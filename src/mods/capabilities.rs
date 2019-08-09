extern crate twox_hash;
extern crate walkdir;

use self::walkdir::{WalkDir};

use diesel::prelude::*;
use diesel::sql_query;

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

use Models::{Listing};

// use rusqlite::{Connection, Result, NO_PARAMS};

pub enum ChecksumState {
    NotPresent,
    PresentButNoChecksum,
    PresentWithChecksum
}

pub fn delete_listing_tag(conn: &PgConnection, listing_tag_id: &str) {
    Sql::delete_listing_tag(conn, listing_tag_id);
}

pub fn delete_missing_listings(conn: &PgConnection) {
    println!("Scanning for missing files.");

    match find_missing(&conn) {
        None => println!("none missing"),
        Some(x) =>
            {
                println!("some missing");
                for y in x {
                    println!("removing listing for missing file: {}", y.file_path);
                    Sql::delete_listing(conn, &y.file_path)
                }
            }
    }
}

pub fn delete_tag(conn: &PgConnection, tag_id: &str) {
    Sql::delete_tag(conn, tag_id);
}

pub fn find_duplicates(conn: &PgConnection) -> Option<Vec<Models::Listing>> {
    let results = sql_query(include_str!("queries/duplicates.sql"))
                    .load::<Listing>(conn);

    match results {
        Err(_error) => {
            println!("Something went wrong with duplicate detection.");
            None
        },
        Ok(rows) => {
            println!("{:?} duplicate files found.", &rows.len());
            Some(rows)
        }
    }
}

pub fn find_tagged_listings(conn: &PgConnection) -> Option<Vec<Models::AppliedTag>> {
    use Models::{AppliedTag};

    let results = sql_query(include_str!("queries/applied_tags.sql"))
                    .load::<AppliedTag>(conn);

    match results {
        Err(_error) => {
            println!("Something went wrong with finding applied tags.");
            None
        },
        Ok(rows) => {
            println!("{:?} listings with applied tags found.", &rows.len());
            Some(rows)
        }
    }
}

pub fn find_missing(conn: &PgConnection) -> Option<Vec<Models::Listing>> {
    use Schema::listings::dsl::*;

    let results = listings
        .load::<Listing>(conn)
        .expect("Error loading listings");

    let mut missing: Vec<Models::Listing> = Vec::new();

    for listing in results {
        if !Util::does_file_exist(&Util::unescape_sql_string(&listing.file_path)) {
            missing.push(listing);
        }
    }

    match missing.len() {
        0 => None,
        _ => Some(missing)
    }
}

pub fn hash_file(file_name: &str) -> String {
    let path = Path::new(&file_name);
    if !path.exists() {
        return String::from("Not Found!");
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

pub fn is_file_hashed(file_path_to_check: &str, conn: &rusqlite::Connection) -> (ChecksumState, Option<String>) {
    let results = Sql::find_single_file(conn, file_path_to_check);

    if results.is_empty(){
        (ChecksumState::NotPresent, None)
    } else if results[0].checksum == None {
        (ChecksumState::PresentButNoChecksum, Some(results.into_iter().nth(0).expect("Could not get listing id").id))
    } else {
        (ChecksumState::PresentWithChecksum, Some(results.into_iter().nth(0).expect("Could not get listing id").id))
    }
}

pub fn start_hashing(root_directory: &str, conn: &rusqlite::Connection) {
    let walker = WalkDir::new(root_directory).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if Util::is_dir(&entry) {
            // do nothing
        } else {
            let file_path = entry.path().display().to_string();

            let is_hashed: (ChecksumState, Option<String>) = is_file_hashed(&Util::escape_sql_string(&file_path), &conn);

            match is_hashed {
                (ChecksumState::NotPresent, None) =>
                {
                    // TODO 18-09-22 Need to research diesel error checking and see if strings can be cleaned up
                    println!("hashing new file: {}", &file_path);
                    match Sql::create_listing(
                                &conn,
                                &hash_file(&file_path),
                                &Util::escape_sql_string(&entry.file_name().to_str().unwrap().to_string()),
                                &Util::escape_sql_string(&entry.path().display().to_string()),
                                &Util::get_file_len(&file_path)) {
                                    Ok(_x) => (),
                                    Err(_error) => panic!("Insert failed"),
                                };
                }
                (ChecksumState::PresentButNoChecksum, Some(x)) =>
                {
                    println!("hashing previously logged: {}", &file_path);
                    Sql::update_hash(&conn, &x, &hash_file(&file_path));
                }
                (ChecksumState::PresentWithChecksum, Some(_)) => {println!("skipping hash for: {}", &file_path)}
                (_, _) => {}
            }
        }
    }
}

pub fn create_tag(conn: &PgConnection, tag: &str) {
    Sql::create_tag(conn, tag);
}

pub fn create_listing_tag(conn: &PgConnection, listing_id: &str, tag_id: &str) {
    Sql::create_listing_tag(conn, listing_id, tag_id);
}

pub fn tag_listing(conn: &PgConnection, listing_id: &str, tag_name: &str) {
    use Models::Tag;

    // since tags and listing tags will be made if not existant, we can take advantage of the create SQL
    let the_tag: Tag = Sql::create_tag(conn, tag_name);
    Sql::create_listing_tag(conn, listing_id, &the_tag.id);
}