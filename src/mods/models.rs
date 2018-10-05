// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

use mods::schema::listings;

#[derive(Queryable, QueryableByName)]
#[table_name="listings"]
pub struct Listing {
    pub id: i32,
    pub checksum: Option<String>,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64
}

#[derive(Insertable)]
#[table_name="listings"]
pub struct NewListing<'a> {
    pub checksum: &'a str,
    pub file_name: &'a str,
    pub file_path: &'a str,
    pub file_size: &'a i64
}