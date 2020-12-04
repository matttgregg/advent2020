use pest::Parser;
use pest_derive::Parser;

use std::time::SystemTime;

#[derive(Parser)]
#[grammar = "parsers/day4.pest"]
pub struct DParser {}

pub fn run() {
    println!("Day4!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data4.txt");
    let contents = String::from_utf8_lossy(cbytes);

    let (total, valid, valid2) = parse_file(&contents);
    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();

    println!(
        "Found {}/{} 'valid' passports out of {}.",
        valid, valid2, total
    );
    println!("Timed: {}us", timed);
}

fn is_between(val: &str, min: i32, max: i32) -> usize {
    let parsed = val.parse::<i32>();
    if let Ok(v) = parsed {
        if v >= min && v <= max {
            1
        } else {
            0
        }
    } else {
        0
    }
}

pub fn parse_file(unparsed_file: &str) -> (i32, i32, i32) {
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
            if f.as_rule() != Rule::cid {
                fcount += 1;
                let mut parts = f.into_inner();
                let try_part = parts.next();
                if let Some(part) = try_part {
                    if parts.next() == None {
                        fcount2 += match part.as_rule() {
                            Rule::vbirth => is_between(part.as_str(), 1920, 2002),
                            Rule::vissue => is_between(part.as_str(), 2010, 2020),
                            Rule::vexpire => is_between(part.as_str(), 2020, 2030),
                            Rule::cmheight => is_between(part.as_str(), 150, 193),
                            Rule::inheight => is_between(part.as_str(), 59, 76),
                            Rule::vhair | Rule::vecl | Rule::vpid => 1,
                            _ => 0,
                        }
                    }
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
    (total, valid, valid2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_test() {
        let cbytes = include_bytes!("../data/data4.txt");
        let contents = String::from_utf8_lossy(cbytes);
        assert_eq!((296, 239, 188), parse_file(&contents));
    }
}
