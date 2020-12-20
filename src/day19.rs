use pest::Parser;
use pest_derive::Parser;

use termion::cursor;

use std::collections::HashMap;
use std::time::SystemTime;

use advent2020::{fmt_bright, fmt_red, fmt_green, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data19.txt")
}

pub fn run() {
    print_day(19);

    let start = SystemTime::now();

    let data_small = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
    parse(&data_small);
    parse(data());

    // Let's do this...

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

#[derive(Debug)]
enum RuleFragment {
    Ref(usize),
    Literal(char)
}

type RuleSequence = Vec<RuleFragment>;

#[derive(Debug)]
struct RuleOptions {
    options: Vec<RuleSequence>
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<usize, RuleOptions>
}

impl Rules {
    fn new() -> Self {
        Rules {rules: HashMap::new()}
    }

    fn add_rule(&mut self, index: usize, rule: RuleOptions) {
        self.rules.insert(index, rule);
    }
    
    fn matches(&self, index: usize, text: &str, patched: bool, trying: usize) -> Option<(String, String)> {
        if !patched {
            self.inner_matches(index, text, patched, trying)
        }
        else if index == 8 {
            // We greedily match rule 42
            let mut matched = String::from("");
            let mut remainder = String::from(text);
            let mut ncount = 0;
            loop {
                if ncount >= trying {
                    break
                } else if let Some((new_match, new_remainder)) = self.inner_matches(42, &remainder, patched, trying) {
                    ncount += 1;
                    matched.push_str(&new_match);
                    remainder = new_remainder;
                } else {
                    break;
                }
            }
            println!("--MATCHED 42x{}", ncount);

            if matched.is_empty() {
                None
            } else {
                Some((matched, remainder))
            }
            /*
        } else if index == 11 {
            // Can look like 42{n} 31{n} where n >= 1;
            // Collect 42s
            let mut matched = String::from("");
            let mut remainder = String::from(text);
            let mut n = 0;
            loop {
                if let Some((new_match, new_remainder)) = self.inner_matches(42, &remainder, patched) {
                    matched.push_str(&new_match);
                    remainder = new_remainder;
                    n += 1;
                } else {
                    break;
                }
            }

            if matched.is_empty() {
                return None;
            }

            // Now, we *must* match n instances of 31
            for _ in 0..n {
                if let Some((new_match, new_remainder)) = self.inner_matches(31, &remainder, patched) {
                    matched.push_str(&new_match);
                    remainder = new_remainder;
                } else {
                    return None; // *Couldn't match up the right number of matching 31s.
                }
            }
            Some((matched, remainder))
            */
        } else {
            let mut to_try = trying;
            loop {
                let res = self.inner_matches(index, text, patched, trying);
                if to_try > 2 || res != None {
                    break res
                }
                to_try += 1;
            }
        }
    }

    fn inner_matches(&self, index: usize, text: &str, patched: bool, trying: usize) -> Option<(String, String)> {
        if let Some(r) = self.rules.get(&index) {
            for option in &r.options {
                // We attempt to match each option in turn.
                let mut matched = String::from("");
                let mut remainder = String::from(text);
                let mut failed = false;
                for frag in option {
                    match frag {
                        RuleFragment::Literal(c) => {
                            if let Some(ch) = remainder.chars().next() {
                                if ch == *c {
                                    matched.push(ch);
                                    remainder = remainder[1..].to_string();
                                } else {
                                    failed = true;
                                    break;
                                }
                            } else {
                                failed = true;
                                break;
                            }
                        },
                        RuleFragment::Ref(i) => {
                            // Attempt to match against the sub expression.
                            if let Some((submatch, subremainder)) = self.matches(*i, &remainder, patched, trying) {
                                matched.push_str(&submatch);
                                remainder = subremainder;
                            } else {
                                failed = true;
                                break;
                            }
                        },
                    };
                }
                if !failed {
                    // We managed to match against this option!
                    return Some((matched, remainder));
                }
            }
        }

        None
    }
}



pub fn parse(transmission: &str) {
    let file = DParser::parse(Rule::file, transmission)
        .expect("unsuccesful parse")
        .next()
        .unwrap();

    let mut data = vec![];
    let mut rules: Rules = Rules::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::datum => { data.push(line.as_str()); },
            Rule::rule => {
                // We're reading a rule.
                let mut index: Option<usize> = None;
                let mut options = vec![];
                for parts in line.into_inner() {
                    match parts.as_rule() {
                        Rule::index => { index = Some(parts.as_str().parse().unwrap()); },
                        Rule::seq => {
                            // Unpack the sequence.
                            let mut seq = vec![];
                            for seq_part in parts.into_inner() {
                                match seq_part.as_rule() {
                                    Rule::literal => {
                                        seq.push(RuleFragment::Literal(seq_part.as_str().chars().next().unwrap()));
                                    },
                                    Rule::sub => {
                                        seq.push(RuleFragment::Ref(seq_part.as_str().parse().unwrap()));
                                    },
                                    _ => panic!("Unexpected part of squence."),
                                }
                            }
                            options.push(seq);
                        },
                        _ => panic!("unexpected part of rule")
                    }
                }
                if let Some(ix) = index {
                    rules.add_rule(ix, RuleOptions{ options });
                } else {
                    panic!("Failed to parse the rule.");
                }
            },
            // Silently consume EOI.
            Rule::EOI => {},
            _ => {
                println!("Unexpected rule: {:?} {}", line.as_rule(), line.as_str());
                panic!("Unexpected rule");
            },
        };
    };


    println!("Read rules: {:?}", rules);
    let mut match_count = 0;
    let mut extended_match_count = 0;
    for datum in data {
        if let Some((matched, remainder)) = rules.matches(0, datum, false, 1) {
            println!("{}{}", fmt_green(&matched), fmt_red(&remainder));
            if remainder.is_empty() {
                match_count += 1;
            }
        } else {
            println!("{:50}", fmt_red(&datum));
        }
        
        if let Some((matched, remainder)) = rules.matches(0, datum, true, 1) {
            println!("{}{}", fmt_green(&matched.to_ascii_uppercase()), fmt_red(&remainder.to_ascii_uppercase()));
            if remainder.is_empty() {
                extended_match_count += 1;
            }
        } else {
            println!("{:50}", fmt_red(&datum.to_ascii_uppercase()));
        }
    }
    println!("Found {} matches.", fmt_bright(&match_count));
    println!("Found {} extended matches.", fmt_bright(&extended_match_count));
} 

#[derive(Parser)]
#[grammar = "parsers/day19.pest"]
pub struct DParser {}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
