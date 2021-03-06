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
    let prog = parse_fast(code);

    let loop_value = match run_program(&prog, false) {
        ProgramResult::Looped(acc) | ProgramResult::Terminated(acc) => acc,
    };

    match run_program(&prog, true) {
        ProgramResult::Terminated(fixed) => (loop_value, fixed),
        ProgramResult::Looped(_) => (loop_value, -1),
    }
}

fn parse_fast(code: &str) -> Vec<Instruction> {
    let mut program = vec![];

    for line in code.lines() {
        let instruction = match &line[0..3] {
            "nop" => {
                let (unpacked, sign) = parse_signed(&line[4..]);
                Instruction::Nop(unpacked, sign)
            }
            "jmp" => {
                let (unpacked, sign) = parse_signed(&line[4..]);
                Instruction::Jump(unpacked, sign)
            }
            "acc" => Instruction::Acc(line[4..].parse::<i64>().unwrap()),
            _ => Instruction::Ignored,
        };
        program.push(instruction);
    }
    program
}

fn parse_signed(v: &str) -> (usize, bool) {
    let sign = match v.chars().next() {
        Some('+') => true,
        Some('-') => false,
        _ => panic!("Unexpected string format."),
    };
    (v[1..].parse::<usize>().unwrap(), sign)
}

enum ProgramResult {
    Looped(i64),
    Terminated(i64),
}

enum Instruction {
    Jump(usize, bool),
    Nop(usize, bool),
    Acc(i64),
    Ignored,
}

fn run_program(program: &[Instruction], flip: bool) -> ProgramResult {
    let seen: HashSet<usize> = HashSet::new();
    let counter: usize = 0;
    let acc = 0;
    run_program_from(program, flip, &seen, counter, acc)
}

fn run_program_from(
    program: &[Instruction],
    flip: bool,
    init_seen: &HashSet<usize>,
    mut counter: usize,
    mut acc: i64,
) -> ProgramResult {
    // Need to copy the loop breaker.
    let mut seen: HashSet<usize> = init_seen.iter().copied().collect();

    return loop {
        if seen.contains(&counter) {
            break ProgramResult::Looped(acc);
        }
        seen.insert(counter);

        if let Some(ins) = program.get(counter) {
            match ins {
                Instruction::Acc(val) => {
                    acc += val;
                    counter += 1;
                }
                Instruction::Jump(val, sign) => {
                    if flip {
                        // Try flipping to nop to get a result.
                        if let terminal @ ProgramResult::Terminated(_) =
                            run_program_from(program, false, &seen, counter + 1, acc)
                        {
                            break terminal;
                        }
                    }

                    // Flip was unsuccesful, so continue.
                    if *sign {
                        counter = counter.saturating_add(*val);
                    } else {
                        counter = counter.saturating_sub(*val);
                    }
                }
                Instruction::Nop(val, sign) => {
                    if flip {
                        // Try flipping to jump to get a result.
                        let shifted = if *sign {
                            counter.saturating_add(*val)
                        } else {
                            counter.saturating_sub(*val)
                        };
                        if let terminal @ ProgramResult::Terminated(_) =
                            run_program_from(program, false, &seen, shifted, acc)
                        {
                            break terminal;
                        }
                    }

                    // Flip was unsuccesful, so continue.
                    counter += 1;
                }
                Instruction::Ignored => {
                    counter += 1;
                }
            }
        } else {
            return ProgramResult::Terminated(acc);
        }
    };
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!((1475, 1270), solve_program(data()));
    }
}
