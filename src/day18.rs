use pest::Parser;
use pest_derive::Parser;

use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data18.txt")
}

pub fn run() {
    print_day(18);

    let start = SystemTime::now();

    // Let's do this...
    let total = eval_file(data());
    let total_priority = eval_file_priority(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Grand total with no priority = {}", fmt_bright(&total));
    println!("Grand total with addition priority = {}", fmt_bright(&total_priority));
    print_duration(timed);
}

pub fn eval_file(data: &str) -> i64 {
    data.lines().map(eval).sum()
}


pub fn eval_file_priority(data: &str) -> i64 {
    data.lines().map(eval_priority).sum()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum CalcPart {
    Num(i64),
    Plus,
    Times
}

pub fn eval_priority(code: &str) -> i64 {
    // In priotity mode, we evaluate a list of op/values, then can apply "+" before "-"
    let calc = DParser::parse(Rule::calc, code)
        .expect("unsuccesful parse")
        .next()
        .unwrap();

    let mut calc_parts: Vec<CalcPart> = vec![];

    for part in calc.into_inner() {
        let atom = match part.as_rule() {
            Rule::num => CalcPart::Num(part.as_str().parse().unwrap()),
            Rule::calc => CalcPart::Num(eval_priority(part.as_str())),
            Rule::plus => CalcPart::Plus,
            Rule::times => CalcPart::Times,
            _ => panic!("unexpected match"),
        };
        calc_parts.push(atom);
    }

    // Now reduce '+'
    loop {
        let mut changes = 0;
        let mut index = 0;
        let mut reduced_plus = vec![];
    loop {
        if index + 2 >= calc_parts.len() {
            if index < calc_parts.len() {
                reduced_plus.push(calc_parts[index]);
                if index + 1 < calc_parts.len() {
                    reduced_plus.push(calc_parts[index + 1]);
                    changes += 1;
                }
            }
            break;
        }

        if calc_parts[index + 1] == CalcPart::Plus {
            if let CalcPart::Num(lhs) = calc_parts[index]  {
                if let CalcPart::Num(rhs) = calc_parts[index + 2]  {
                    reduced_plus.push(CalcPart::Num(lhs + rhs));
                    index += 3;
                } else {
                    panic!("Expected number.");
                }
            } else {
                panic!("Expected number.");
            }
        } else {
            reduced_plus.push(calc_parts[index]);
            reduced_plus.push(calc_parts[index + 1]);
            index += 2;
        }
    }
        calc_parts = reduced_plus;
        if changes == 0 {
            break;
        }
    }

    // We *should* just have multiples left, so can just multiply all remaining numbers.
    let mut product = 1;
    for cp in calc_parts {
        if let CalcPart::Num(num) = cp {
            product *= num;
        }
    }

    product
}


pub fn eval(code: &str) -> i64 {
    let calc = DParser::parse(Rule::calc, code)
        .expect("unsuccesful parse")
        .next()
        .unwrap();

    let mut running: Option<i64> = None;
    let mut current_op: Option<char> = None;

    for part in calc.into_inner() {
        match part.as_rule() {
            Rule::num => {
                let val: i64 = part.as_str().parse().unwrap();
                match running {
                    None => { running = Some(val); },
                    Some(r) => {
                        // We have an existing value, so apply the current op.
                        match current_op {
                            Some('+') => { running = Some(r + val); }
                            Some('*') => { running = Some(r * val); }
                            _ => panic!("Attempt to apply non-op"),
                        }
                    }
                }

            },
            Rule::calc => {
                let val: i64 = eval(part.as_str());
                match running {
                    None => { running = Some(val); },
                    Some(r) => {
                        // We have an existing value, so apply the current op.
                        match current_op {
                            Some('+') => { running = Some(r + val); }
                            Some('*') => { running = Some(r * val); }
                            _ => panic!("Attempt to apply non-op"),
                        }
                    }
                }

            },
            Rule::plus => {current_op = Some('+');},
            Rule::times => {current_op = Some('*');},
            _ => panic!("unexpected match"),
        }

    }

    match running {
        Some(v) => v,
        None => panic!("Could not evaluate.")
    }
}

#[derive(Parser)]
#[grammar = "parsers/day18.pest"]
pub struct DParser {}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(51, eval(&"1 + (2 * 3) + (4 * (5 + 6))"));
        
    }

    #[test]
    fn test_part2() {
        assert_eq!(231, eval_priority("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(23340, eval_priority("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }

    #[test]
    fn test_all() {
        assert_eq!(1408133923393, eval_file(data()));
        assert_eq!(314455761823725, eval_file_priority(data()));
    }
}
