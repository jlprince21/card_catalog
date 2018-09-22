# File Scanner

Written in Rust, this program collects file metadata and stores it in a PostgreSQL
database. Some things it gathers include:

1. File name
2. Path
3. Size
4. XxHash checksum

# Getting Started

To run the project, you will need a PostgreSQL database setup and configure the
`.env` file in this project to point to your database.

Next, you will need to use [diesel](http://diesel.rs/) to run the database migrations
necessary to create tables needed for the project.

```
diesel migration run
```

For now you will have to change in `main.rs` which folder you would like to start
scan. Configuration reading will be updated to use `.env` soon.

Once the prerequisites are met, you may run or build the project with:

```
# To run
cargo run

# To build for release
cargo build --release

```

# License

License is MIT. See LICENSE.