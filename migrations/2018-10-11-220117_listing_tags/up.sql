CREATE TABLE listing_tags (
    id text PRIMARY KEY,
    listing_id text references listings(id) NOT NULL,
    tag_id text references tags(id) NOT NULL
);