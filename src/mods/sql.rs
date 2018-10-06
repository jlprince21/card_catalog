use diesel;
use diesel::prelude::*;

use Models::{NewListing, Listing};

pub fn create_listing(conn: &PgConnection, checksum: &str, file_name: &str, file_path: &str, file_size: &i64) -> Listing {
    use mods::schema::listings;

    let new_listing = NewListing {
        checksum,
        file_name,
        file_path,
        file_size,
    };

    diesel::insert_into(listings::table)
        .values(&new_listing)
        .get_result(conn)
        .expect("Error saving new listing")
}

pub fn delete_listing(conn: &PgConnection, p_file_path: &str) {
    use Schema::listings::dsl::*;

    // 18-09-23 TODO: One day, may want to mark listings as deleted instead of removing them
    diesel::delete(listings.filter(file_path.eq(p_file_path)))
        .execute(conn)
        .expect("Error deleting listing");
}

pub fn establish_connection(connection: &str) -> PgConnection {
    PgConnection::establish(&connection).unwrap_or_else(|_| panic!("Error connecting to {}", connection))
}

pub fn find_single_file(conn: &PgConnection, p_file_path: &str) -> Vec<Listing> {
    use Schema::listings::dsl::*;
    listings
        .filter(file_path.eq(p_file_path))
        .limit(1)
        .load::<Listing>(conn)
        .expect("Error loading posts")
}

pub fn update_hash(conn: &PgConnection, id: i32, hash: &str) {
    use super::schema::listings::dsl::{listings, checksum};
    diesel::update(listings.find(id))
        .set(checksum.eq(hash))
        .get_result::<Listing>(conn)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));
}