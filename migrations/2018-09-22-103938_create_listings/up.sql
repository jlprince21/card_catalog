CREATE TABLE listings (
    id SERIAL PRIMARY KEY,
    checksum text,
    file_name text NOT NULL,
    file_path text NOT NULL,
    file_size bigint NOT NULL
)