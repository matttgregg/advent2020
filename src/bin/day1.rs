use std::env;
use std::process;

use advent2020::ConfigD1 as Config;
use advent2020::run_d1 as run;

fn main() {
    println!("Day1!");
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
