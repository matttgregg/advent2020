use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename given"),
        };

        Ok(Config { filename, })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let contents: HashSet<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap_or(0)).collect();

    // Find sums to 2020
    for val in contents.iter() {
        let complement = 2020 - val;
        if val < &complement && contents.contains(&complement) {
            println!("{} x {} -> {}", val, complement, val * complement);
        }

        if val < &complement {
            sums_to_part(&contents, &val);
        }
    }
    Ok(())
}

fn sums_to_part(vals: &HashSet<i32>, part: &i32) {
    for val in vals.iter() {
        let complement = 2020 - val - part;
        if part < val && val < &complement && vals.contains(&complement) {
            println!("{} x {} x {} -> {}", part, val, complement, part * val * complement);
        }
    }
}
