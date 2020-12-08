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
    let (initial, fixed) = solve_program(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Initial program loops at {}", fmt_bright(&initial));
    println!("Fixed program terminates with {}", fmt_bright(&fixed));
    print_duration(timed);
}

fn solve_program(code: &str) -> (i64, i64) {
    let prog = parse(code);

    let loop_value = match run_program(&prog, 0) {
        ProgramResult::Looped(acc) | ProgramResult::Terminated(acc) => acc,
    };

    let mut fix_at = 1;
    let fix_value = loop {
        if let ProgramResult::Terminated(acc) = run_program(&prog, fix_at) {
            break acc
        }
        fix_at += 1;
    };

    (loop_value, fix_value)
}

fn parse(code: &str) -> Vec<(&str, i64)> {
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
            program.push((op, val));
        }
    }

    program
}

enum ProgramResult {
    Looped(i64),
    Terminated(i64),
}

fn run_program(program: &[(&str, i64)], mut flip_counter: usize) -> ProgramResult  {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut counter: usize = 0;
    let mut acc = 0;

    loop {
        if seen.contains(&counter) {
            break;
        }
        seen.insert(counter);

        let instr = program.get(counter);
        if let Some((op, val)) = instr {
            match *op {
                "acc" => {
                        acc += val;
                        counter += 1;
                }
                "jmp" => {
                    if flip_counter == 1 {
                        // Flip to a no-op.
                        counter += 1;
                    } else {
                    counter += *val as usize;
                    }

                    flip_counter = flip_counter.saturating_sub(1);
                },
                _ => {
                    if flip_counter == 1 {
                        // Flip to a jump
                        counter += *val as usize;
                    } else {
                        counter += 1;
                    }

                    flip_counter = flip_counter.saturating_sub(1);
                },
            }
        } else {
            return ProgramResult::Terminated(acc);
        }
    }

    ProgramResult::Looped(acc)
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
