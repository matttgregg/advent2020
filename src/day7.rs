use pest::Parser;
use pest_derive::Parser;

use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

pub fn run() {
    print_day(7);

    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data7.txt");
    let contents = String::from_utf8_lossy(cbytes);

    let res = parse_bags(&contents);

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("{} bag types contain shiny gold bags.", fmt_bright(&res));

    print_duration(timed);
}

pub fn parse_bags(data: &str) -> usize {
    let mut bag_contains: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut contains_gold = HashSet::new();

    let parsed = DParser::parse(Rule::file, data)
        .expect("could not parse data")
        .next()
        .unwrap();

    for line in parsed.into_inner() {
        let mut line_parts = line.into_inner();
        // First part is the outer bag
        let bag = line_parts.next();
        let bag_name = match bag {
            Some(x) => x.as_str(),
            None => "",
        };

        for sub in line_parts {
            if sub.as_rule() == Rule::somebags {
                let mut bits = sub.into_inner();
                let bag_bit = bits.next();
                let _count = bag_bit.unwrap().as_str().parse::<usize>().unwrap();
                let what = bits.next().unwrap().as_str();
                if what == "shiny gold" {
                    contains_gold.insert(bag_name);
                }

                if contains_gold.contains(what) {
                    contains_gold.insert(bag_name);
                }
                bag_contains.entry(bag_name).or_insert(vec![]).push(what);
            }
        }
    }

    let mut no_gold = HashSet::new();

    for (bag, contents) in &bag_contains {
        let mut working = vec![];
        for inner in contents {
            working.push(inner.to_owned());
        }

        while !working.is_empty() {
            let check = working.pop();
            if check == Some("shiny gold") {
                // We've hit gold! Note and continue.
                contains_gold.insert(bag);
                break;
            }

            if let Some(c) = check {
                if contains_gold.contains(&c) {
                    // This bag already contains gold somewhere.
                    contains_gold.insert(bag);
                    break;
                }

                // We don't have to bother checking if we've already checked this bag has no gold.
                if bag_contains.contains_key(c) && !no_gold.contains(&c) {
                    for inner in &bag_contains[c] {
                        working.push(inner);
                    }
                }
            }

            if working.is_empty() {
                // This bag contains absolutely no gold.
                no_gold.insert(bag);
            }
        }
    }

    contains_gold.len()
}

#[derive(Parser)]
#[grammar = "parsers/day7.pest"]
pub struct DParser {}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {}
}
