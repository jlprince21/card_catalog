# File Scanner

Written in Rust, this program collects file metadata and stores it in a MariaDB
database. Some things it gathers include:

1. File name
2. Path
3. Size
4. XxHash checksum

# Getting Started

To run the project, you will need to have a MariaDB SQL database running somewhere
with the tables setup according to the `Listings.sql` file in the `SQL` folder.
Set appropriate values in `settings.toml` to point to your database and the
root directory to recursively scan files of.

Once the prerequisites are met, you may run or build the project with:

```
# To run
cargo run

# To build for release
cargo build --release

```

# License

License is MIT. See LICENSE.