// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

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

extern crate card_catalog;
use card_catalog::cc;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let settings: Util::Settings = Util::get_settings();
    let action = matches.value_of("action").unwrap_or("none");

    if let Some(matches) = matches.subcommand_matches("new-tag") {
        // cargo run -- new-tag puppy
        let tag = matches.value_of("tag").unwrap_or("none");
        println!(r#"Creating new tag "{}"."#, tag);
        cc::new_tag(tag);
        std::process::exit(0);
    }

    if let Some(matches) = matches.subcommand_matches("delete-tag-listing") {
        // cargo run -- delete-tag-listing 56982fc3-091a-489c-bd6c-c7f916965d4b
        let listing_tag_id = matches.value_of("listing-tag-id").unwrap_or("none");
        println!("Deleting tag listing with id {}.", listing_tag_id);
        cc::delete_tag_listing(listing_tag_id);
        std::process::exit(0);
    }

    if let Some(matches) = matches.subcommand_matches("delete-tag") {
        // cargo run -- delete-tag 56982fc3-091a-489c-bd6c-c7f916965d4b
        let tag_id = matches.value_of("tag-id").unwrap_or("none");
        println!("Deleting tag with id {} and all associated listing_tags.", tag_id);
        cc::delete_tag(tag_id);
        std::process::exit(0);
    }

    if let Some(matches) = matches.subcommand_matches("tag") {
        //  Example: cargo run -- tag cf80775e-3c25-4c3d-87f6-290357359bb8 -- summer beach vacation
        let listing_id = value_t!(matches.value_of("id"), String).unwrap_or_else(|e| e.exit()); // handy macro from clap
        let tags: Vec<_> = matches.values_of("tags").unwrap().collect();
        cc::tag(&listing_id, tags);
        println!("Tag(s) applied successfully!");
        std::process::exit(0);
    }

    match action {
        "duplicates" => {
            cc::duplicates();
        },
        "hash" => {
            cc::hash(&settings.directory_to_scan);
        },
        "orphans" => {
            cc::orphans();
        }
        _ => {
            println!("No valid args provided, exiting.");
            std::process::exit(0);
        }
    };
}