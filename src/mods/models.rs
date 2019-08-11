// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

use time::Timespec;

#[derive(Debug)]
pub struct Listing {
    pub id: String,
    pub checksum: Option<String>,
    pub time_created: Timespec,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub tag: String
}

#[derive(Debug, Clone)]
pub struct ListingTag {
    pub id: String,
    pub listing_id: String,
    pub tag_id: String
}

#[derive(Debug)]
pub struct AppliedTag {
    pub listing_id: String,
    pub checksum: Option<String>,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub listing_tags_id: String,
    pub tags_id: String,
    pub tag: String
}