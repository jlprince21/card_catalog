table! {
    listing_tags (id) {
        id -> Int4,
        listing_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    listings (id) {
        id -> Int4,
        checksum -> Nullable<Text>,
        file_name -> Text,
        file_path -> Text,
        file_size -> Int8,
    }
}

table! {
    tags (id) {
        id -> Int4,
        tag -> Text,
    }
}

joinable!(listing_tags -> listings (listing_id));
joinable!(listing_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    listing_tags,
    listings,
    tags,
);
