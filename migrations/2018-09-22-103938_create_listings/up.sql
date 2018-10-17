CREATE TABLE listings (
    id text PRIMARY KEY,
    checksum text,
    file_name text NOT NULL,
    file_path text NOT NULL,
    file_size bigint NOT NULL
)