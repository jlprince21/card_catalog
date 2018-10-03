// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

extern crate config;

#[macro_use]
extern crate diesel;

pub mod mods;
use mods::util as Util;
use mods::models as Models;
use mods::sql as Sql;
use mods::schema as Schema;
use mods::capabilities as Capabilities;

fn main() {
    let settings: Util::Settings = Util::get_settings();

    let connection = Sql::establish_connection(&settings.pg_connection_string);

    // Capabilities::start_hashing(&settings.directory_to_scan, &connection);
    // Capabilities::delete_missing_listings(&connection);
    // Capabilities::find_duplicates(&connection);
}