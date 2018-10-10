#![feature(plugin)]
#![plugin(rocket_codegen)]

// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

extern crate clap;
extern crate config;

extern crate rocket_cors;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};

#[macro_use]
extern crate diesel;

use clap::{Arg, App};

#[macro_use] extern crate rocket_contrib;

pub mod mods;
use mods::util as Util;
use mods::models as Models;
use mods::sql as Sql;
use mods::schema as Schema;
use mods::capabilities as Capabilities;

extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/duplicates")]
fn duplicates() -> Json<Vec<Models::Listing>> {
    let settings: Util::Settings = Util::get_settings();
    let connection = Sql::establish_connection(&settings.pg_connection_string);

    println!("Searching for duplicate files...");

    let res = Capabilities::find_duplicates(&connection);
    let emp: Vec<Models::Listing> = Vec::new();

    match res {
        None => Json(emp),
       Some(x) => Json(x)
    }
}

#[get("/")]
fn cors<'a>() -> &'a str {
    "Hello CORS"
}

fn main() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:4200"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![index, duplicates])
        .attach(options)
        .launch();

    // let matches = App::new("File Scanner")
    //                     .version("0.0.1")
    //                     .author("Luke Prince github.com/jlprince21")
    //                     .about("Assists in indexing a collection of files")
    //                     .arg(Arg::with_name("action")
    //                         .short("a")
    //                         .long("action")
    //                         .value_name("ACTION")
    //                         .help("Select which action you want the file scanner to perform")
    //                         .takes_value(true))
    //                     .get_matches();

    // let settings: Util::Settings = Util::get_settings();
    // let connection = Sql::establish_connection(&settings.pg_connection_string);
    // let action = matches.value_of("action").unwrap_or("none");

    // match action {
    //     "duplicates" => {
    //         println!("Searching for duplicate files...");
    //         Capabilities::find_duplicates(&connection);
    //     },
    //     "hash" => {
    //         println!("Hashing...");
    //         Capabilities::start_hashing(&settings.directory_to_scan, &connection);
    //     },
    //     "orphans" => {
    //         println!("Removing orphans from database...");
    //         Capabilities::delete_missing_listings(&connection);
    //     }
    //     _ => {
    //         println!("No valid args provided, exiting.");
    //         std::process::exit(0);
    //     }
    // };
}