#![warn(clippy::all, clippy::pedantic)]
use std::env;
use std::process;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    match day_arg(env::args()) {
        Some(1) => day1::run(),
        Some(2) => day2::run(),
        Some(3) => day3::run(),
        Some(4) => day4::run(),
        Some(0) => {
            day1::run();
            day2::run();
            day3::run();
            day4::run();
        }
        Some(x) => { eprintln!("Unimplemented day {}", x); process::exit(1);},
        None => { eprintln!("No valid day on command line {:#?}", env::args()); process::exit(1);},
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

    None
}
