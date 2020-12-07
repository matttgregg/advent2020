use pest::Parser;
use pest_derive::Parser;

use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> String {
    let cbytes = include_bytes!("../data/data7.txt");
    String::from_utf8_lossy(cbytes).to_string()
}

pub fn run() {
    print_day(7);

    let start = SystemTime::now();

    let (contains_gold, in_gold) = parse_bags(&data(), "shiny gold");

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!(
        "{} bag types contain shiny gold bags.",
        fmt_bright(&contains_gold)
    );
    println!(
        "Each gold shiny bag contains {} bags in total.",
        fmt_bright(&in_gold)
    );

    print_duration(timed);
}

pub fn parse_bags(data: &str, my_bag_type: &str) -> (usize, usize) {
    let mut contains_gold = HashSet::new();

    let mut contains: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();

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
                let count = bag_bit.unwrap().as_str().parse::<usize>().unwrap();
                let what = bits.next().unwrap().as_str();
                if what == my_bag_type {
                    contains_gold.insert(bag_name);
                }

                if contains_gold.contains(what) {
                    contains_gold.insert(bag_name);
                }
                contains
                    .entry(bag_name)
                    .or_insert(vec![])
                    .push((what, count));
            }
        }
    }

    let mut no_gold = HashSet::new();

    for (bag, contents) in &contains {
        let mut working = vec![];
        for (inner, _) in contents {
            working.push(inner.to_owned());
        }

        while !working.is_empty() {
            let check = working.pop();
            if check == Some(my_bag_type) {
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
                if contains.contains_key(c) && !no_gold.contains(&c) {
                    for (inner, _) in &contains[c] {
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

    // Work out the gold contents
    let mut gold_count = 0;
    let mut gold_working = vec![];

    for (bag, count) in contains.entry(my_bag_type).or_default() {
        gold_working.push((*bag, *count));
    }

    loop {
        let mut repacked = vec![];
        // We unpack each bag in turn.
        for (bags, count) in &gold_working {
            gold_count += count;

            let sub_bags = contains.get(bags);

            if let Some(subs) = sub_bags {
                for (ibag, icount) in subs {
                    repacked.push((*ibag, *icount * count));
                }
            }
        }

        gold_working.clear();

        for (bag, count) in repacked {
            gold_working.push((bag, count));
        }

        if gold_working.is_empty() {
            break;
        }
    }

    (contains_gold.len(), gold_count)
}

#[derive(Parser)]
#[grammar = "parsers/day7.pest"]
pub struct DParser {}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!((169, 82372), parse_bags(&data(), "shiny gold"));
    }
}
