#![warn(clippy::all, clippy::pedantic)]
use std::env;
use std::process;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    match day_arg(env::args()) {
        Some(1) => day1::run(),
        Some(2) => day2::run(),
        Some(3) => day3::run(),
        Some(4) => day4::run(),
        Some(5) => day5::run(),
        Some(6) => day6::run(),
        Some(7) => day7::run(),
        Some(8) => day8::run(),
        Some(9) => day9::run(),
        Some(0) => {
            day1::run();
            day2::run();
            day3::run();
            day4::run();
            day5::run();
            day6::run();
            day7::run();
            day8::run();
            day9::run();
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
