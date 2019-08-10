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

use Models::{Listing, ListingTwo};

use rusqlite::{Connection, Result, NO_PARAMS};

pub enum ChecksumState {
    NotPresent,
    PresentButNoChecksum,
    PresentWithChecksum
}

pub fn delete_listing_tag(conn: &rusqlite::Connection, listing_tag_id: &str) {
    match Sql::delete_listing_tag(conn, listing_tag_id) {
        Ok(x) => {
            println!("Deleted {} listing tags with id '{}'", x, listing_tag_id);
        },
        Err(_err) => {
            println!("Error deleting listing tag with id '{}'", listing_tag_id);
        }
    }
}

pub fn delete_missing_listings(conn: &mut rusqlite::Connection) {
    println!("Scanning for missing files.");

    match find_missing(&conn) {
        None => println!("none missing"),
        Some(listings) =>
            {
                println!("some missing");
                for curr_listing in listings {
                    println!("removing listing for missing file: {}", curr_listing.file_path);
                    match Sql::delete_listing(conn, &curr_listing) {
                        Ok(x) => {
                            println!("Deleted listing with id {}", curr_listing.id);
                        },
                        Err(_err) => {
                            println!("Error deleting listing with id {}", curr_listing.id);
                        }
                    }
                }
            }
    }
}

pub fn delete_tag(conn: &mut rusqlite::Connection, tag_id: &str) {
    match Sql::delete_tag(conn, tag_id) {
        Ok(x) => {
            println!("Tag with id {} deleted", tag_id);
        },
        Err(_err) => {
            println!("Error deleting tag with id {}", tag_id);
        }
    }
}

pub fn find_duplicates(conn: &rusqlite::Connection) -> Option<Vec<Models::ListingTwo>> {
    // TODO 19-08-10 move query to a file
    let mut stmt = match conn
        .prepare("SELECT * FROM listing
WHERE
    checksum IN (
        SELECT
            checksum
        FROM (
            SELECT
                checksum,
                ROW_NUMBER()
                OVER (PARTITION BY
                        checksum
                    ORDER BY
                        id ASC) AS Row
                FROM
                    listing) dups
            WHERE
                dups.Row > 1)")
        {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when checking for duplicates")},
        };

    let listing_iter = match stmt
        .query_map(NO_PARAMS, |row| Ok(ListingTwo {
            id: row.get(0)?,
            checksum: row.get(1)?,
            time_created: row.get(2)?,
            file_name: row.get(3)?,
            file_path: row.get(4)?,
            file_size: row.get(5)?
        })) {
            Ok(x) => {
                x
            },
            Err(_error) => {
                panic!("Failed to load results from query")
            },
        };

    // TODO 19-08-08 this is all a little hacky and may be condensable to one or two lines
    let mut duplicate_listings: Vec<ListingTwo> = Vec::new();

    for listing in listing_iter {
        duplicate_listings.insert(0, listing.unwrap());
    }

    println!("{:?} duplicate files found.", &duplicate_listings.len());

    match duplicate_listings.len() {
        0 => {
            None
        },
        _ => {
            Some(duplicate_listings)
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

pub fn find_missing(conn: &rusqlite::Connection) -> Option<Vec<Models::ListingTwo>> {
     let mut stmt = match conn
        .prepare("SELECT id, checksum, time_created, file_name, file_path, file_size FROM listing") {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when gathering listings")},
        };

    let listing_iter = match stmt
        .query_map(NO_PARAMS, |row| Ok(ListingTwo {
            id: row.get(0)?,
            checksum: row.get(1)?,
            time_created: row.get(2)?,
            file_name: row.get(3)?,
            file_path: row.get(4)?,
            file_size: row.get(5)?,
        })) {
            Ok(x) => {
                x
            },
            Err(_error) => {
                panic!("Failed to load results from query")
            },
        };

    // TODO 19-08-08 this is all a little hacky and may be condensable to one or two lines
    let mut missing: Vec<Models::ListingTwo> = Vec::new();

    for listing in listing_iter {
        let the_listing: ListingTwo = listing.unwrap();

        if !Util::does_file_exist(&Util::unescape_sql_string(&the_listing.file_path)) {
            println!("Found missing {}", &the_listing.file_path);
            missing.push(the_listing);
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

pub fn create_tag(conn: &rusqlite::Connection, tag: &str) {
    Sql::create_tag(conn, tag);
}

pub fn create_listing_tag(conn: &rusqlite::Connection, listing_id: &str, tag_id: &str) {
    Sql::create_listing_tag(conn, listing_id, tag_id);
}

pub fn tag_listing(conn: &rusqlite::Connection, listing_id: &str, tag_name: &str) {
    use Models::TagTwo;

    // since tags and listing tags will be made if not existant, we can take advantage of the create SQL
    let the_tag: TagTwo = Sql::create_tag(conn, tag_name).unwrap();
    Sql::create_listing_tag(conn, listing_id, &the_tag.id);
}
