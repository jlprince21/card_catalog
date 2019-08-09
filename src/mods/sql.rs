use diesel;
use diesel::prelude::*;

use uuid::Uuid;

extern crate rusqlite;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS, params};

use Models::{NewTag, Tag, NewListingTag, ListingTag, ListingTwo};

pub fn create_listing(conn: &rusqlite::Connection, checksum: &str, file_name: &str, file_path: &str, file_size: &i64) -> Result<()> {

    // TODO 19-08-08 the checksum here may need to have an alteration in event of being empty; not sure if this code is safe
    let new_listing = ListingTwo {
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

pub fn create_listing_tag(conn: &PgConnection, p_listing_id: &str, p_tag_id: &str) -> ListingTag {
    use Schema::listing_tags;
    use Schema::listing_tags::dsl::*;

    let new_listing_tag = NewListingTag {
        id: &Uuid::new_v4().to_string(),
        listing_id: &p_listing_id,
        tag_id: &p_tag_id
    };

    let single_listing_tag: Vec<ListingTag> = listing_tags
                                                .filter(listing_id.eq(p_listing_id))
                                                .filter(tag_id.eq(p_tag_id))
                                                .limit(1)
                                                .load::<ListingTag>(conn)
                                                .expect("Error loading listing tag");

    match single_listing_tag.len() {
        0 => {
                // TODO 18-10-13 Need proper error handling for when inserts fail eg foreign key violations.
                diesel::insert_into(listing_tags::table)
                    .values(&new_listing_tag)
                    .get_result(conn)
                    .expect("Error saving new listing tag")
            },
        _ => {
            let result: ListingTag = single_listing_tag.into_iter().nth(0).expect("Failed to load existing listing tag.");
            result
        }
    }
}

pub fn create_tag(conn: &PgConnection, p_tag: &str) -> Tag {
    use Schema::tags;
    use Schema::tags::dsl::*;

    let new_tag = NewTag {
        id: &Uuid::new_v4().to_string(),
        tag: p_tag
    };

    let single_tag: Vec<Tag> = tags
                                .filter(tag.eq(p_tag))
                                .limit(1)
                                .load::<Tag>(conn)
                                .expect("Error loading tag");

    match single_tag.len() {
        0 => {
            // TODO 18-10-13 Need proper error handling for when inserts fail eg foreign key violations.
            diesel::insert_into(tags::table)
                .values(&new_tag)
                .get_result(conn)
                .expect("Error saving new tag")
            },
        _ => {
            let result: Tag = single_tag.into_iter().nth(0).expect("Failed to load existing tag.");
            result
        }
    }
}

pub fn delete_listing(conn: &PgConnection, p_file_path: &str) {
    use Schema::listings::dsl::*;

    // TODO 18-09-23 One day, may want to mark listings as deleted instead of removing them
    diesel::delete(listings.filter(file_path.eq(p_file_path)))
        .execute(conn)
        .expect("Error deleting listing");
}

/// Deletes a listing tag while leaving associated tag untouched.
pub fn delete_listing_tag(conn: &PgConnection, p_listing_tag_id: &str) {
    use Schema::listing_tags::dsl::*;

    // TODO 18-10-20 One day, may want to mark listing_tags as deleted instead of removing them
    diesel::delete(listing_tags.filter(id.eq(p_listing_tag_id)))
        .execute(conn)
        .expect("Error deleting listing tag");
}

pub fn delete_tag(conn: &PgConnection, p_tag_id: &str) {
    // TODO 18-10-20 Feels hacky putting these deletes in blocks to avoid ambiguity on the id column... would like to see this nicer
    // TODO 18-10-20 One day, may want to mark listing_tags and tags as deleted instead of removing them
    {
        use Schema::listing_tags::dsl::*;
        diesel::delete(listing_tags.filter(tag_id.eq(p_tag_id)))
            .execute(conn)
            .expect("Error deleting listing tag");
    }
    {
        use Schema::tags::dsl::*;
        diesel::delete(tags.filter(id.eq(p_tag_id)))
            .execute(conn)
            .expect("Error deleting tag");
    }
}

pub fn establish_connection(connection: &str) -> rusqlite::Connection {
    rusqlite::Connection::open(&connection).unwrap_or_else(|_| panic!("Error connecting to {}", connection))
}

pub fn find_single_file(conn: &rusqlite::Connection, p_file_path: &str) -> Vec<ListingTwo> {
    let mut stmt = match conn
        .prepare(&format!("SELECT id, checksum, time_created, file_name, file_path, file_size FROM listing where file_path='{}'", &p_file_path))
        {
            Ok(x) => {x},
            Err(_error)=> { panic!("Error connecting to database when checking if file hashed")},
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
    let mut single_listing: Vec<ListingTwo> = Vec::new();

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