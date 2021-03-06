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
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        Some(10) => day10::run(),
        Some(11) => day11::run(),
        Some(12) => day12::run(),
        Some(13) => day13::run(),
        Some(14) => day14::run(),
        Some(15) => day15::run(),
        Some(16) => day16::run(),
        Some(17) => day17::run(),
        Some(18) => day18::run(),
        Some(19) => day19::run(),
        Some(20) => day20::run(),
        Some(21) => day21::run(),
        Some(22) => day22::run(),
        Some(23) => day23::run(),
        Some(24) => day24::run(),
        Some(25) => day25::run(),
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
            day10::run();
            day11::run();
            day12::run();
            day13::run();
            day14::run();
            day15::run();
            day16::run();
            day17::run();
            day18::run();
            day19::run();
            day20::run();
            day21::run();
            day22::run();
            day23::run();
            day24::run();
            day25::run();
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
