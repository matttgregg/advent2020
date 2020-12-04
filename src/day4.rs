use pest::Parser;
use pest_derive::Parser;

use std::time::SystemTime;

/// byr (Birth Year)
/// iyr (Issue Year)
///    eyr (Expiration Year)
///    hgt (Height)
///    hcl (Hair Color)
///    ecl (Eye Color)
///    pid (Passport ID)
///    cid (Country ID)

#[derive(Parser)]
#[grammar = "parsers/day4.pest"]
pub struct DParser {}

pub fn run() {
    println!("Day4!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data4.txt");
    let contents = String::from_utf8_lossy(cbytes);

    parse_file(&contents);
    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();
    println!("Timed: {}us", timed);
}

fn is_between(val: &str, min: i32, max: i32) -> bool {
    let parsed = val.parse::<i32>();
    if let Ok(v) = parsed {
        v >= min && v <= max
    } else {
        false
    }
}

pub fn parse_file(unparsed_file: &str) {
    let file = DParser::parse(Rule::file, unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    let mut valid = 0;
    let mut valid2 = 0;
    let mut total = 0;
    for line in file.into_inner() {
        let mut fcount = 0;
        let mut fcount2 = 0;
        for f in line.into_inner() {
            fcount += 1;
            let rule = f.as_rule();
            let mut parts = f.into_inner();
            let try_part = parts.next();
            match rule {
                Rule::birth => {
                    if let Some(part) = try_part {
                        if part.as_rule() == Rule::vbirth && parts.next() == None && is_between(part.as_str(), 1920, 2002) {
                            fcount2 += 1;
                        }
                    }
                },
                Rule::issue => {
                    if let Some(part) = try_part {
                        if part.as_rule() == Rule::vissue && parts.next() == None && is_between(part.as_str(), 2010, 2020) {
                            fcount2 += 1;
                        }
                    }
                },
                Rule::expire => {
                    if let Some(part) = try_part {
                        if part.as_rule() == Rule::vexpire && parts.next() == None && is_between(part.as_str(), 2020, 2030) {
                            fcount2 += 1;
                        }
                    }
                },
                Rule::height => {
                    if let Some(part) = try_part {
                        if ((part.as_rule() == Rule::cmheight && is_between(part.as_str(), 150, 193)) ||
                           (part.as_rule() == Rule::inheight && is_between(part.as_str(), 59, 76))) && parts.next() == None {
                                fcount2 += 1;
                        }
                    }
                },
                Rule::hair => {
                    if let Some(part) = try_part {
                        if part.as_rule() == Rule::vhair && parts.next() == None {
                            fcount2 += 1;
                        }
                    }
                },
                Rule::eyes => {
                    if let Some(part) = try_part {
                        if part.as_rule() == Rule::vecl && parts.next() == None {
                            fcount2 += 1;
                        }
                    }
                },
                Rule::pid => {
                    if let Some(part) = try_part {
                        if part.as_rule() == Rule::vpid && parts.next() == None {
                            fcount2 += 1;
                        }
                    }
                },
                Rule::cid => {
                    fcount -= 1;
                }
                _ => {
                    fcount += 1;
                    fcount2 += 1;
                }
            }
        }

        if fcount == 7 {
            valid += 1;
        }
        if fcount2 == 7 {
            valid2 += 1;
        }
        if fcount > 0 {
            total += 1;
        }
    }

    println!(
        "Found {}/{} valid passports out of {}.",
        valid, valid2, total
    );
}
