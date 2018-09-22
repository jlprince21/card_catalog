table! {
    listings (id) {
        id -> Int4,
        checksum -> Nullable<Text>,
        file_name -> Text,
        file_path -> Text,
        file_size -> Int8,
    }
}
