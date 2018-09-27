use diesel;
use diesel::prelude::*;

use Models::{NewListing, Listing};

pub fn create_listing(conn: &PgConnection, checksum: &str, file_name: &str, file_path: &str, file_size: &i64) -> Listing {
    use mods::schema::listings;

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

pub fn establish_connection(connection: &String) -> PgConnection {
    PgConnection::establish(&connection).expect(&format!("Error connecting to {}", connection))
}