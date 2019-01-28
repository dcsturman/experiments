use std::env;

use clap::App;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let yaml =  load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        let query = matches.value_of("QUERY").unwrap().to_string();
        let filename = matches.value_of("FILENAME").unwrap().to_string();

        let case_sensitive:bool = if matches.is_present("case_insensitive") {
            let ci = matches.value_of("case_insensitive").unwrap();
            let ci:bool = ci.parse().unwrap();
            !ci
        } else {
            env::var("CASE_INSENSITIVE").is_err()
        };

        Ok(Config { query, filename, case_sensitive })
    }
}
