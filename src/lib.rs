// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

#![crate_type = "lib"]

#[macro_use]
extern crate clap;
extern crate config;

#[macro_use]
extern crate diesel;

extern crate rusqlite;
extern crate time;
extern crate uuid;

pub mod schema;
use ::schema as Schema;

pub mod mods;
use mods::util as Util;
use mods::models as Models;
use mods::sql as Sql;
use mods::capabilities as Capabilities;

pub mod cc {
    use Capabilities;
    use Sql;
    use Util;
    use rusqlite::{Connection};

    fn get_connection() -> Connection {
        let settings: Util::Settings = Util::get_settings();
        Sql::establish_connection(&settings.sqlite_connection_string)
    }

    pub fn duplicates() {
        println!("Searching for duplicate files...");
        // Capabilities::find_duplicates(&get_connection());
    }

    pub fn hash(root_directory: &str) {
        println!("Hashing...");
        Capabilities::start_hashing(&root_directory, &get_connection());
    }

    pub fn orphans() {
        println!("Removing orphans from database...");
        // Capabilities::delete_missing_listings(&get_connection());
    }

    pub fn tag(listing_id: &str, tags: Vec<&str>) {
        for tag in tags {
            Capabilities::tag_listing(&get_connection(), &listing_id, tag);
        }
        println!("Tag(s) applied successfully!");
        std::process::exit(0);
    }

    pub fn delete_tag(tag_id: &str) {
        // Capabilities::delete_tag(&get_connection(), tag_id);
    }

    pub fn new_tag(tag: &str) {
        Capabilities::create_tag(&get_connection(), tag);
    }

    pub fn delete_tag_listing(listing_tag_id: &str) {
        Capabilities::delete_listing_tag(&get_connection(), listing_tag_id);
    }
}