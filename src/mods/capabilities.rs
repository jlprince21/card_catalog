extern crate twox_hash;
extern crate walkdir;

use self::walkdir::{WalkDir};

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

use Models::{Listing, AppliedTag};

use rusqlite::{Connection, NO_PARAMS, Result};

use std::fs;

pub enum ChecksumState {
    NotPresent,
    PresentButNoChecksum,
    PresentWithChecksum
}

pub fn delete_listing_tag(conn: &Connection, listing_tag_id: &str) {
    match Sql::delete_listing_tag(conn, listing_tag_id) {
        Ok(x) => {
            println!("Deleted {} listing tags with id '{}'", x, listing_tag_id);
        },
        Err(_err) => {
            println!("Error deleting listing tag with id '{}'", listing_tag_id);
        }
    }
}

pub fn delete_missing_listings(conn: &mut Connection) {
    println!("Scanning for missing files.");

    match find_missing(&conn) {
        None => println!("No orphans found"),
        Some(listings) =>
            {
                for curr_listing in listings {
                    match Sql::delete_listing(conn, &curr_listing) {
                        Ok(_x) => {
                            println!("Deleted missing listing with id {}", curr_listing.id);
                        },
                        Err(_err) => {
                            println!("Error deleting missing listing with id {}", curr_listing.id);
                        }
                    }
                }
            }
    }
}

pub fn delete_tag(conn: &mut Connection, tag_id: &str) {
    match Sql::delete_tag(conn, tag_id) {
        Ok(_x) => {
            println!("Tag with id {} deleted", tag_id);
        },
        Err(_err) => {
            println!("Error deleting tag with id {}", tag_id);
        }
    }
}

pub fn find_duplicates(conn: &Connection) -> Option<Vec<Models::Listing>> {
    let query: String = match fs::read_to_string("src/mods/queries/duplicates.sql") {
        Ok(_x) => {
            _x
        },
        Err(_err) => {
            panic!("Error preparing duplicates query")
        }
    };

    let mut stmt = match conn
        .prepare(&query)
        {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when checking for duplicates")},
        };

    let listing_iter = match stmt
        .query_map(NO_PARAMS, |row| Ok(Listing {
            id: row.get("id")?,
            checksum: row.get("checksum")?,
            time_created: row.get("time_created")?,
            file_name: row.get("file_name")?,
            file_path: row.get("file_path")?,
            file_size: row.get("file_size")?
        })) {
            Ok(x) => {
                x
            },
            Err(_error) => {
                panic!("Failed to load results from query")
            },
        };

    // TODO 19-08-08 this is all a little hacky and may be condensable to one or two lines
    let mut duplicate_listings: Vec<Listing> = Vec::new();

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

pub fn find_tagged_listings(conn: &Connection) -> Option<Vec<Models::AppliedTag>> {
    // TODO 19-08-10 move query to a file
    let query: String = match fs::read_to_string("src/mods/queries/applied_tags.sql") {
        Ok(_x) => {
            _x
        },
        Err(_err) => {
            panic!("Error preparing duplicates query")
        }
    };

    let mut stmt = match conn
        .prepare(&query)
        {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when checking for applied tags")},
        };

    let applied_tag_iter = match stmt
        .query_map(NO_PARAMS, |row| Ok(AppliedTag {
            listing_id: row.get("listing_id")?,
            checksum: row.get("checksum")?,
            file_name: row.get("file_name")?,
            file_path: row.get("file_path")?,
            file_size: row.get("file_size")?,
            listing_tag_id: row.get("listing_tag_id")?,
            tag_id: row.get("tag_id")?,
            tag: row.get("tag")?,
        })) {
            Ok(x) => {
                x
            },
            Err(_error) => {
                panic!("Failed to load results from query")
            },
        };

    // TODO 19-08-08 this is all a little hacky and may be condensable to one or two lines
    let mut applied_tags: Vec<AppliedTag> = Vec::new();

    for applied in applied_tag_iter {
        applied_tags.insert(0, applied.unwrap());
    }

    println!("{:?} applied tags found.", &applied_tags.len());

    match applied_tags.len() {
        0 => {
            None
        },
        _ => {
            Some(applied_tags)
        }
    }
}

pub fn find_missing(conn: &Connection) -> Option<Vec<Models::Listing>> {
     let mut stmt = match conn
        .prepare("SELECT id, checksum, time_created, file_name, file_path, file_size FROM listing") {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when gathering listings")},
        };

    let listing_iter = match stmt
        .query_map(NO_PARAMS, |row| Ok(Listing {
            id: row.get("id")?,
            checksum: row.get("checksum")?,
            time_created: row.get("time_created")?,
            file_name: row.get("file_name")?,
            file_path: row.get("file_path")?,
            file_size: row.get("file_size")?,
        })) {
            Ok(x) => {
                x
            },
            Err(_error) => {
                panic!("Failed to load results from query")
            },
        };

    // TODO 19-08-08 this is all a little hacky and may be condensable to one or two lines
    let mut missing: Vec<Models::Listing> = Vec::new();

    for listing in listing_iter {
        let the_listing: Listing = listing.unwrap();

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

pub fn is_file_hashed(file_path_to_check: &str, conn: &Connection) -> (ChecksumState, Option<String>) {
    let results = Sql::find_single_file(conn, file_path_to_check);

    if results.is_empty(){
        (ChecksumState::NotPresent, None)
    } else if results[0].checksum == None {
        (ChecksumState::PresentButNoChecksum, Some(results.into_iter().nth(0).expect("Could not get listing id").id))
    } else {
        (ChecksumState::PresentWithChecksum, Some(results.into_iter().nth(0).expect("Could not get listing id").id))
    }
}

pub fn setup(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE listing (
	        id	TEXT NOT NULL,
	        checksum	TEXT,
	        time_created	TEXT NOT NULL,
	        file_name	TEXT NOT NULL,
	        file_path	TEXT NOT NULL,
	        file_size	INTEGER,
	        PRIMARY KEY(id)
        )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE tag (
	        id	TEXT,
	        tag	TEXT NOT NULL,
	        PRIMARY KEY(id)
        )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE listing_tag (
	        id	TEXT,
	        listing_id	TEXT NOT NULL,
	        tag_id	TEXT NOT NULL,
	        PRIMARY KEY(id),
	        FOREIGN KEY(listing_id) REFERENCES listing(id),
            FOREIGN KEY(tag_id) REFERENCES tag(id)
        )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn start_hashing(root_directory: &str, conn: &Connection) {
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

pub fn create_tag(conn: &Connection, tag: &str) {
    Sql::create_tag(conn, tag);
}

pub fn create_listing_tag(conn: &Connection, listing_id: &str, tag_id: &str) {
    Sql::create_listing_tag(conn, listing_id, tag_id);
}

pub fn tag_listing(conn: &Connection, listing_id: &str, tag_name: &str) {
    use Models::Tag;

    // since tags and listing tags will be made if not existant, we can take advantage of the create SQL
    let the_tag: Tag = Sql::create_tag(conn, tag_name).unwrap();
    Sql::create_listing_tag(conn, listing_id, &the_tag.id);
}
