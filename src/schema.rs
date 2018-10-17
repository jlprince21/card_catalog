table! {
    listing_tags (id) {
        id -> Text,
        listing_id -> Text,
        tag_id -> Text,
    }
}

table! {
    listings (id) {
        id -> Text,
        checksum -> Nullable<Text>,
        file_name -> Text,
        file_path -> Text,
        file_size -> Int8,
    }
}

table! {
    tags (id) {
        id -> Text,
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
