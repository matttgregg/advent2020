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
    println!("{}Day {} !{:50}{}", color::Bg(color::Blue), d, "", color::Bg(color::Reset));
}

pub fn print_duration(d: time::Duration) {
    println!("{}", style::Invert);
    println!("Timed: {}us", d.as_micros());
    println!("{}", style::Reset);
}

pub fn fmt_bright<T: std::fmt::Display>(t: &T) -> String {
    format!("{}{}{}", color::Fg(color::LightWhite), t, color::Fg(color::Reset))
}
