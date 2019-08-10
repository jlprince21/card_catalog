// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

use Schema::listings;
use Schema::listing_tags;
use Schema::tags;

use diesel::sql_types::{Text, Nullable, BigInt};

use time::Timespec;

#[derive(Queryable, QueryableByName)]
#[table_name="listings"]
pub struct Listing {
    pub id: String,
    pub checksum: Option<String>,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64
}

#[derive(Insertable)]
#[table_name="listings"]
pub struct NewListing<'a> {
    pub id: &'a str,
    pub checksum: &'a str,
    pub file_name: &'a str,
    pub file_path: &'a str,
    pub file_size: &'a i64
}

#[derive(Queryable, QueryableByName)]
#[table_name="tags"]
pub struct Tag {
    pub id: String,
    pub tag: String
}

#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag<'a> {
    pub id: &'a str,
    pub tag: &'a str,
}

#[derive(Queryable, QueryableByName)]
#[table_name="listing_tags"]
pub struct ListingTag {
    pub id: String,
    pub listing_id: String,
    pub tag_id: String
}

#[derive(Insertable)]
#[table_name="listing_tags"]
pub struct NewListingTag<'a> {
    pub id: &'a str,
    pub listing_id: &'a str,
    pub tag_id: &'a str,
}

// TODO 18-10-21 This could be a view instead; see https://stackoverflow.com/a/51896910
#[derive(QueryableByName)]
pub struct AppliedTag {
    #[sql_type = "Text"]
    pub listing_id: String,
    #[sql_type = "Nullable<Text>"]
    pub checksum: Option<String>,
    #[sql_type = "Text"]
    pub file_name: String,
    #[sql_type = "Text"]
    pub file_path: String,
    #[sql_type = "BigInt"]
    pub file_size: i64,
    #[sql_type = "Text"]
    pub listing_tags_id: String,
    #[sql_type = "Text"]
    pub tags_id: String,
    #[sql_type = "Text"]
    pub tag: String
}

// TODO 19-08-08 models below this line will need renaming as old models above are replaced

#[derive(Debug)]
pub struct ListingTwo {
    pub id: String,
    pub checksum: Option<String>,
    pub time_created: Timespec,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64
}

#[derive(Debug, Clone)]
pub struct TagTwo {
    pub id: String,
    pub tag: String
}

#[derive(Debug, Clone)]
pub struct ListingTagTwo {
    pub id: String,
    pub listing_id: String,
    pub tag_id: String
}