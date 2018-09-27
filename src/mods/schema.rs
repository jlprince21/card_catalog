// suppress warnings for issue #50504 <https://github.com/rust-lang/rust/issues/50504>
#![allow(proc_macro_derive_resolution_fallback)]

table! {
    listings (id) {
        id -> Int4,
        checksum -> Nullable<Text>,
        file_name -> Text,
        file_path -> Text,
        file_size -> Int8,
    }
}
