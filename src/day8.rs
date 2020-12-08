use pest::Parser;
use pest_derive::Parser;

use std::collections::HashSet;
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data8.txt")
}

pub fn run() {
    print_day(8);

    let start = SystemTime::now();

    // Let's do this...
    parse(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn parse(code: &str) {

    let parsed = DParser::parse(Rule::file, code)
        .expect("could not parse data")
        .next()
        .unwrap();

    let mut program = vec![];

    for line in parsed.into_inner() {
        if line.as_rule() == Rule::instruction {
        let mut instr = line.into_inner();
        let op = instr.next();
        let op = op.unwrap().as_str();
        let val = instr.next();
        let val = val.unwrap().as_str().parse::<i64>().unwrap_or(0);
        println!("Op : {} , Val : {}", op, val);
            program.push((op, val));
        }
    }

    let mut seen: HashSet<usize> = HashSet::new();
    let mut counter: usize = 0;
    let mut acc = 0;

    loop {
        if seen.contains(&counter) {
            break;
        }
        seen.insert(counter);

        let (op, val) = program[counter];
        match op {
            "acc" => {
                acc += val;
                counter += 1;
            },
            "jmp" => {
                counter += val as usize;
            },
            _ => {
                counter += 1;
            },
        }
    }

    println!("Final acc: {}", acc)
     
}

#[derive(Parser)]
#[grammar = "parsers/day8.pest"]
pub struct DParser {}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
