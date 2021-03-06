use std::env;
use std::time;
use termion::{color, style};

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
        }
;

        Ok(Config { filename, })
    }
}

pub fn print_day(d: i32) {
    println!(" \u{1F389} {}Day {} !{}", style::Underline, d, style::Reset);
}

pub fn crab() -> String {
    String::from("\u{1F980}")
}

pub fn print_duration(d: time::Duration) {
    if d.as_micros() < 1000 {
    println!(" \u{1F44D} {}Timed: {}us {}", style::Invert, d.as_micros(), style::Reset);
    } else {
        println!(" \u{1F44D} {}Timed: {} ms {}us {}", style::Invert, d.as_millis(), d.as_micros() % 1000, style::Reset);
    }

    println!{}; 
}

pub fn fmt_bright<T: std::fmt::Display>(t: &T) -> String {
    format!("{}{}{}", color::Fg(color::LightWhite), t, color::Fg(color::Reset))
}

pub fn fmt_red<T: std::fmt::Display>(t: &T) -> String {
    format!("{}{}{}", color::Fg(color::Red), t, color::Fg(color::Reset))
}

pub fn fmt_green<T: std::fmt::Display>(t: &T) -> String {
    format!("{}{}{}", color::Fg(color::Green), t, color::Fg(color::Reset))
}
