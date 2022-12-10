# This project is no longer under active development. The project is still public since it is available on crates.io. Use at your own risk.

# Card Catalog

[![Build Status](https://travis-ci.org/jlprince21/card_catalog.svg?branch=master)](https://travis-ci.org/jlprince21/card_catalog)

Written in Rust, this program collects file metadata and stores it in a SQLite
database. Some things it gathers include:

1. File name
2. Path
3. Size
4. XxHash checksum

In addition to collecting data about files, the program assists in indexing files
with tools such as file tagging, search, and more. Development on these features
is underway... stay tuned!

# Getting Started

To run the project, you will need a SQLite database setup and configure the
`.env` file in this project to point to your database file. See *Configuration*
section below.

Next, you will need to create the requisite tables by running the database setup
process

```
cargo run -- --action setup
```

Next, set any remaining configuration values as detailed in *Configuration*.

Once the prerequisites are met, you may run or build the project with:

```
# To see help
cargo run -- --help

# To run
cargo run

# To build for release
cargo build --release

```

See *Arguments* section for details on the arguments this program accepts.

# Arguments

This app takes a minimum of one command line argument before it will perform any
action beyond simply terminating. This section is divided into commands and
subcommands.

## Commands

### Action

You can specify one of several actions to use via the `-a` or `--action` command
flags followed by an action name. For now configuration beyond selecting an
action to perform is handled in the `.env` file, see *Configuration*. Valid
actions are:

1. duplicates - finds duplicate files within database via **matching** hashes.
2. hash - computes hashes, file size, etc and stores results in database.
3. orphans - iterates *all* database entries computed by hash action and does
a simple check to see if files are still present. If a file is not present, its
entry in the database will be removed.

Examples:

```
# Start hashing files:
cargo run -- --action hash
```

## Subcommands

### Creating a Tag

To create a tag without applying it to a listing, eg "puppy" use

```
cargo run -- new-tag puppy
```

### Tagging a Listing

To aid in searching for any given file, you can apply tags to a listing id which
in the future will be used as a search mechanism. For example, you could search
for all files containing the tag `vacation` and viola :violin:, all files with
the tag applied are returned!

To tag a listing, whose id is _56982fc3-091a-489c-bd6c-c7f916965d4b_, with tags
of `summer`, `beach`, and `vacation`:

```
cargo run -- tag 56982fc3-091a-489c-bd6c-c7f916965d4b -- summer beach vacation
```

### Removing a Tag from a Listing

To remove a single tag applied to a listing, use the UUID in the id column of
`listing_tags` to remove the applied tag association.

```
cargo run -- delete-tag-listing 56982fc3-091a-489c-bd6c-c7f916965d4b
```

### Deleting a Tag

Deleting a tag will remove it from the `tags` table and all entries of where the
tag was in use on the `listing_tags` table. Proceed with caution! To make this a
little harder to accidentally run, for now tags must be deleted with their UUID
in the id column within the `tags` table.

```
cargo run -- delete-tag 56982fc3-091a-489c-bd6c-c7f916965d4b
```

# Configuration

`.env` configuration setting include:

1. DIRECTORY_TO_SCAN - root directory location to start scanning files from.
2. SQLITE_CONNECTION - path where SQLite database file will be read/write from.

# More

See `documentation` folder for more information.

# License

License is MIT. See LICENSE file.
