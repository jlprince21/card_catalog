CREATE TABLE listing_tags (
    id SERIAL PRIMARY KEY,
    listing_id int references listings(id) NOT NULL,
    tag_id int references tags(id) NOT NULL
);