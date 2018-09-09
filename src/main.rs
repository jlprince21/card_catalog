extern crate config;
extern crate mysql;

use mysql as my;

// derive line enables easy printing of struct
#[derive(Debug)]
struct Listing {
    file_name: String
}

fn main() {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("settings"))
        .expect("Config file missing!");

    let sql_user = format!("{}", settings.get_str("SQLUser").unwrap());
    let sql_password = format!("{}", settings.get_str("SQLPassword").unwrap());
    let sql_server = format!("{}", settings.get_str("SQLServer").unwrap());
    let sql_port = format!("{}", settings.get_str("SQLPort").unwrap());
    let sql_database = format!("{}", settings.get_str("SQLDatabase").unwrap());

    let connection_string = format!("mysql://{}:{}@{}:{}/{}", sql_user, sql_password, sql_server, sql_port, sql_database);
    println!("{}", connection_string);

    let pool = my::Pool::new(connection_string).unwrap();

    let selected_listings: Vec<Listing> =
    pool.prep_exec("SELECT FileName from Listings limit 10", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Listing>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Listing`
        result.map(|x| x.unwrap()).map(|row| {
            // ⚠️ Note that from_row will panic if you don't follow your schema
            let FileName = my::from_row(row);
            Listing {
                file_name: FileName,
            }
        }).collect() // Collect Listings so now `QueryResult` is mapped to `Vec<Listing>`
    }).unwrap(); // Unwrap `Vec<Listing>`

    println!("{:?}", selected_listings);
}