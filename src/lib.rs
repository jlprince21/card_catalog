// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

#![crate_type = "lib"]

#[macro_use]
extern crate clap;
extern crate config;

#[macro_use]
extern crate diesel;

extern crate uuid;

use clap::{Arg, App, SubCommand};

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

    pub fn duplicates() {
        let settings: Util::Settings = Util::get_settings();
        let connection = Sql::establish_connection(&settings.pg_connection_string);

        println!("Searching for duplicate files...");
        Capabilities::find_duplicates(&connection);
    }
}