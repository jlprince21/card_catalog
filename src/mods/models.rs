// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

use Schema::listings;
use Schema::listing_tags;
use Schema::tags;

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
