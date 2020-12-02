use std::env;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip filename
        args.next(); // Skip day index

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename given"),
        };

        Ok(Config { filename, })
    }
}
