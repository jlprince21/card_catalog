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

pub fn establish_connection(connection: &str) -> PgConnection {
    PgConnection::establish(&connection).unwrap_or_else(|_| panic!("Error connecting to {}", connection))
}

pub fn update_hash(conn: &PgConnection, id: i32, hash: &str) {
    use super::schema::listings::dsl::{listings, checksum};

    println!("inside update hash: {} {:?}", id, hash);

    diesel::update(listings.find(id))
        .set(checksum.eq(hash))
        .get_result::<Listing>(conn)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));
}