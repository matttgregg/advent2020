use advent2020::Config;

use std::error::Error;
use std::fs;
use regex::Regex;
use std::string::String;

#[derive(Debug)]
struct RuledPassword {
    min: usize,
    max: usize,
    letter: char,
    password: String
}

impl RuledPassword {
    pub fn new(raw: &str) -> Option<RuledPassword> {
        // Ruled passwords look like: "1-3 a: abcde""
        let re = Regex::new(r"^(\d+)-(\d+) (.): ([^ ]+)$").unwrap(); 
        if let Some(cap) = re.captures(raw) {
            Some(RuledPassword{
                min: cap[1].parse::<usize>().unwrap(),
                max: cap[2].parse::<usize>().unwrap(),
                letter: cap[3].chars().next().unwrap(),
                password: cap[4].to_string(),
            })
        } else {
            None
        }
    }

    pub fn valid(&self) -> bool {
        let count = self.password.chars().filter(|x| *x == self.letter).count();
        count >= self.min && count <= self.max
    }

    pub fn valid2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let got_min = match chars.get(self.min - 1) {
            Some(x) => self.letter == *x,
            None => false
        } ;
        let got_max = match chars.get(self.max - 1) {
            Some(x) => self.letter == *x,
            None => false
        } ;

        !(got_min && got_max) && (got_min || got_max)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running day 2 on {}", config.filename);
    println!("Out of {} ...", line_count(&config.filename));
    println!("Valid by method 1: {}", part1(&config.filename));
    println!("Valid by method 2: {}", part2(&config.filename));

    Ok(())
}

fn part1(filename: &str) -> usize {
    let contents = fs::read_to_string(filename);
    if let Ok(c) = contents {
        c.lines().filter_map(|x| RuledPassword::new(x)).filter(|x| x.valid()).count()
    } else {
        0
    }
}

fn part2(filename: &str) -> usize {
    let contents = fs::read_to_string(filename);
    if let Ok(c) = contents {
        c.lines().filter_map(|x| RuledPassword::new(x)).filter(|x| x.valid2()).count()
    } else {
        0
    }
}

fn line_count(filename: &str) -> usize {
    let contents = fs::read_to_string(filename);
    if let Ok(c) = contents {
        c.lines().count()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let d1 = RuledPassword::new("1-3 a: abcde").expect("Failed to parse.");
        let d2 = RuledPassword::new("1-3 b: cdefg").expect("Failed to parse.");
        let d3 = RuledPassword::new("2-9 c: ccccccccc").expect("Failed to parse.");

        assert_eq!(d1.valid(), true);
        assert_eq!(d2.valid(), false);
        assert_eq!(d3.valid(), true);
    }

    #[test]
    fn part2_test() {
        let d1 = RuledPassword::new("1-3 a: abcde").expect("Failed to parse.");
        let d2 = RuledPassword::new("1-3 b: cdefg").expect("Failed to parse.");
        let d3 = RuledPassword::new("2-9 c: ccccccccc").expect("Failed to parse.");

        assert_eq!(d1.valid2(), true);
        assert_eq!(d2.valid2(), false);
        assert_eq!(d3.valid2(), false);
    }

    #[test]
    fn all_test() {
        let file = "./data/data2.txt";
        assert_eq!(538, part1(&file));
        assert_eq!(489, part2(&file));
    }
}
