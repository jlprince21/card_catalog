use uuid::Uuid;

extern crate rusqlite;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS, params};

use Models::{Listing, ListingTag, Tag};

pub fn create_listing(conn: &rusqlite::Connection, checksum: &str, file_name: &str, file_path: &str, file_size: &i64) -> Result<()> {

    // TODO 19-08-08 the checksum here may need to have an alteration in event of being empty; not sure if this code is safe
    let new_listing = Listing {
        id: Uuid::new_v4().to_string(),
        checksum: Some(checksum.to_string()),
        time_created: time::get_time(),
        file_name: file_name.to_string(),
        file_path: file_path.to_string(),
        file_size: *file_size
    };

    conn.execute(
        "INSERT INTO listing
                (id, checksum, time_created, file_name, file_path, file_size)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        &[&new_listing.id as &ToSql,
            &new_listing.checksum as &ToSql,
            &new_listing.time_created,
            &new_listing.file_name as &ToSql,
            &new_listing.file_path as &ToSql,
            &new_listing.file_size as &ToSql],
    )?;

    Ok(())
}

pub fn create_listing_tag(conn: &Connection, p_listing_id: &str, p_tag_id: &str) -> Option<ListingTag> {
    let new_listing_tag = ListingTag {
        id: Uuid::new_v4().to_string(),
        listing_id: p_listing_id.to_string(),
        tag_id: p_tag_id.to_string()
    };

    let mut stmt = match conn
        .prepare(&format!("SELECT id, listing_id, tag_id FROM listing_tag where listing_id = '{}' AND tag_id = '{}'", &p_listing_id, &p_tag_id)) {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when checking if listing is already tagged")},
        };

    let tag_iter = match stmt
        .query_map(NO_PARAMS, |row| Ok(ListingTag {
            id: row.get(0)?,
            listing_id: row.get(1)?,
            tag_id: row.get(2)?,
        })) {
            Ok(x) => {
                x
            },
            Err(_error) => {
                panic!("Failed to load results from query")
            },
        };

    // TODO 19-08-08 this is all a little hacky and may be condensable to one or two lines
    let mut single_listing_tag: Vec<ListingTag> = Vec::new();

    for tag in tag_iter {
        single_listing_tag.insert(0, tag.unwrap());
    }

    match single_listing_tag.len() {
        0 => {
                match conn.execute(
                    "INSERT INTO listing_tag (id, listing_id, tag_id)
                        VALUES(?1, ?2, ?3)",
                    &[&new_listing_tag.id as &ToSql,
                        &new_listing_tag.listing_id as &ToSql,
                        &new_listing_tag.tag_id as &ToSql],
                ) {
                    Ok(_inserted) => {
                        println!("Tag '{}' created and applied to listing '{}'", p_tag_id, p_listing_id);
                        return Some(new_listing_tag);
                    },
                    Err(_err) => {
                        panic!("Failed to create new tag.")
                    }
                }
            },
        _ => {
            println!("listing is already tagged, returning existing tag");
            let result: ListingTag = single_listing_tag.into_iter().nth(0).expect("Failed to load existing listing tag.");
            Some(result)
        }
    }
}

pub fn create_tag(conn: &Connection, p_tag: &str) -> Option<Tag> {
    let new_tag = Tag {
        id: Uuid::new_v4().to_string(),
        tag: p_tag.to_string()
    };

    let mut stmt = match conn
        .prepare(&format!("SELECT id, tag FROM tag where tag = '{}'", &p_tag)) {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when checking if tag exists")},
        };

    let tag_iter = match stmt
        .query_map(NO_PARAMS, |row| Ok(Tag {
            id: row.get(0)?,
            tag: row.get(1)?,
        })) {
            Ok(x) => {
                x
            },
            Err(_error) => {
                panic!("Failed to load results from query")
            },
        };

    // TODO 19-08-08 this is all a little hacky and may be condensable to one or two lines
    let mut single_tag: Vec<Tag> = Vec::new();

    for tag in tag_iter {
        single_tag.insert(0, tag.unwrap());
    }

    if single_tag.len() == 1 {
        println!("Found existing tag");
        return Some(single_tag.first().cloned().unwrap()); // TODO 19-08-09 not a huge fan of cloning, hope to make this better
    }

    match conn.execute(
        "INSERT INTO tag (id, tag)
            VALUES(?1, ?2)",
        &[&new_tag.id as &ToSql,
            &new_tag.tag as &ToSql],
    ) {
        Ok(_inserted) => {
            println!("New tag '{}' created", p_tag);
            return Some(new_tag);
        },
        Err(_err) => {
            panic!("Failed to create new tag.")
        }
    };
}

pub fn delete_listing(conn: &mut rusqlite::Connection, p_listing: &Listing) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("DELETE from listing_tag WHERE listing_id = ?1", &[&p_listing.id])?;
    tx.execute("DELETE from listing WHERE id = ?1", &[&p_listing.id])?;
    tx.commit()
}

/// Deletes a listing tag while leaving associated tag untouched.
pub fn delete_listing_tag(conn: &rusqlite::Connection, p_listing_tag_id: &str) -> Result<usize> {
    let res = conn.execute(
        "DELETE from listing_tag WHERE id = ?1",
        &[&p_listing_tag_id as &ToSql],
    )?;

    Ok(res)
}

pub fn delete_tag(conn: &mut rusqlite::Connection, p_tag_id: &str) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("DELETE from listing_tag WHERE tag_id = ?1", &[&p_tag_id])?;
    tx.execute("DELETE from tag WHERE id = ?1", &[&p_tag_id])?;
    tx.commit()
}

pub fn establish_connection(connection: &str) -> rusqlite::Connection {
    rusqlite::Connection::open(&connection).unwrap_or_else(|_| panic!("Error connecting to {}", connection))
}

pub fn find_single_file(conn: &rusqlite::Connection, p_file_path: &str) -> Vec<Listing> {
    let mut stmt = match conn
        .prepare(&format!("SELECT id, checksum, time_created, file_name, file_path, file_size FROM listing where file_path='{}'", &p_file_path))
        {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when checking if file hashed")},
        };

    let listing_iter = match stmt
        .query_map(NO_PARAMS, |row| Ok(Listing {
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
    let mut single_listing: Vec<Listing> = Vec::new();

    for listing in listing_iter {
        single_listing.insert(0, listing.unwrap());
    }

    single_listing
}

pub fn update_hash(conn: &rusqlite::Connection, id: &str, hash: &str) {
    let query = format!("UPDATE listing SET checksum = '{}' WHERE id = '{}'", &hash, &id).to_string();

    match conn.execute(&query, NO_PARAMS) {
        Ok(updated) => println!("updated hash for {} row with id {}", updated, id),
        Err(_err) => panic!("Unable to find listing with id {}", id),
    }

}