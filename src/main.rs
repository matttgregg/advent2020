use std::env;
use std::process;

mod day1;
mod day2;

fn main() {
    match day_arg(env::args()) {
        Some(1) => run1(),
        Some(2) => run2(),
        Some(x) => { eprintln!("Unimplemented day {}", x); process::exit(1);},
        None => { eprintln!("No valid day on command line {:#?}", env::args()); process::exit(1);},
    }
}

fn run2() {
    println!("Day2");
    let config = advent2020::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = day2::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run1() {
    println!("Day1!");
    let config = advent2020::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = day1::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn day_arg(mut args: env::Args) -> Option<i32> {
    args.next();

    let day = match args.next() {
        Some(day) => day.parse::<i32>(),
        None => return None,
    };

    if let Ok(i) = day {
        return Some(i);
    }

    return None
}
