use pest_derive::Parser;
use pest::Parser;
use std::string::String;
use std::time::SystemTime;

use advent2020::{print_day, print_duration, fmt_bright};

#[derive(Parser)]
#[grammar = "parsers/day2.pest"]
pub struct DParser {}

#[derive(Debug)]
struct RuledPassword {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl RuledPassword {

    pub fn valid(&self) -> bool {
        let count = self.password.chars().filter(|x| *x == self.letter).count();
        count >= self.min && count <= self.max
    }

    pub fn valid2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let got_min = match chars.get(self.min - 1) {
            Some(x) => self.letter == *x,
            None => false,
        };
        let got_max = match chars.get(self.max - 1) {
            Some(x) => self.letter == *x,
            None => false,
        };

        !(got_min && got_max) && (got_min || got_max)
    }
}

pub fn run() {
    print_day(2);
    
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data2.txt");
    let contents = String::from_utf8_lossy(cbytes);

    let (count, valid, valid2) = parse_file(&contents);
    
    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Total:{} Valid1:{} Valid2:{}", count, fmt_bright(&valid), fmt_bright(&valid2));
    print_duration(timed);

}

pub fn parse_file(unparsed_file: &str) -> (usize, usize, usize) {
    let file = DParser::parse(Rule::file, unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    let mut valid = 0;
    let mut count = 0;
    let mut valid2 = 0;

    for line in file.into_inner() {
        if let Rule::rule = line.as_rule() {
            let mut inner = line.into_inner();
            let mut range_inner = inner.next().unwrap().into_inner();
            let min = range_inner
                .next()
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Failed to parse lower bound");
            let max = range_inner
                .next()
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Failed to parse upper bound");

            let letter = inner.next().unwrap().into_inner().next().unwrap();
            let letter = letter.as_str().chars().next().unwrap();

            let password = inner.next().unwrap().as_str().to_string();

            let ruled_password = RuledPassword {
                min,
                max,
                letter,
                password,
            };

            count += 1;
            if ruled_password.valid() {
                valid += 1;
            }
            if ruled_password.valid2() {
                valid2 += 1;
            }
        };
    }

    (count, valid, valid2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part12_test() {
        assert_eq!((1,1,1), parse_file("1-3 a: abcde"));
        assert_eq!((1,0,0), parse_file("1-3 b: cdefg"));
        assert_eq!((1,1,0), parse_file("2-9 c: ccccccccc"));
    }

    #[test]
    fn all_test() {
        let cbytes = include_bytes!("../data/data2.txt");
        let contents = String::from_utf8_lossy(cbytes);
        assert_eq!((1000, 538, 489), parse_file(&contents));
    }
}
