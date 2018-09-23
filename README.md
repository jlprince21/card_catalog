# File Scanner

Written in Rust, this program collects file metadata and stores it in a PostgreSQL
database. Some things it gathers include:

1. File name
2. Path
3. Size
4. XxHash checksum

# Getting Started

To run the project, you will need a PostgreSQL database setup and configure the
`.env` file in this project to point to your database. See *Configuration* section
below.

Next, you will need to use [diesel](http://diesel.rs/) to run the database migrations
necessary to create tables needed for the project.

```
diesel migration run
```

Next, set any remaining configuration values as detailed in *Configuration*.

Once the prerequisites are met, you may run or build the project with:

```
# To run
cargo run

# To build for release
cargo build --release

```

# Configuration

`.env` configuration setting include:

1. DATABASE_URL - PostgreSQL connection string.
2. DIRECTORY_TO_SCAN - root directory location to start scanning files from.

# License

License is MIT. See LICENSE.