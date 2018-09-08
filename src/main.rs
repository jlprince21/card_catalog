extern crate config;

fn main() {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("settings"))
        .expect("Config file missing!");

    // DirectoryToScan = "XXX"
    // SQLDatabase = "XXX"
    // SQLPassword = "XXX"
    // SQLPort = "3306"
    // SQLServer = "localhost"
    // SQLUser = "user"

    let testread = settings.get_str("DirectoryToScan");

    println!("{:?}", settings.get_str("SQLPort"));
    println!("{:?}", &testread);
}
