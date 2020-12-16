use pest::Parser;
use pest_derive::Parser;

use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data16.txt")
}

fn data_small() -> &'static str {
    include_str!("../data/data16_small.txt")
}

pub fn run() {
    print_day(16);

    let start = SystemTime::now();
    println!("For {} data set.", fmt_bright(&"small"));
    parse_tickets(data_small());
    println!("For {} data set.", fmt_bright(&"full"));
    parse_tickets(data());

    // Let's do this...

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u64>
}

impl Ticket {
    fn error_rate(&self, rules: &[TicketRule]) -> u64 {
        let mut rate = 0;
        for value in &self.values {
            let mut ok = false;
            for rule in rules {
                if rule.fits_any(*value) {
                    ok = true;
                    break;
                }
            }
            if !ok {
                // This field could *not* be validated.
                rate += value;
            }
        }
        rate
    }
}

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64
}

impl Range {
    fn fits(&self, v: u64) -> bool {
        v >= self.min && v <= self.max
    }
}

#[derive(Debug)]
struct TicketRule {
    name: String,
    ranges: Vec<Range>
}

impl TicketRule {
    fn fits_any(&self, v: u64) -> bool {
        for r in &self.ranges {
            if r.fits(v) {
                return true;
            }
        }
        false
    }
}

fn parse_tickets(data: &str) {
    let file = DParser::parse(Rule::file, data)
        .expect("unsuccesful parse")
        .next()
        .unwrap();

    let mut rules = vec![];
    let mut tickets = vec![];
    let mut my_ticket = Ticket { values:vec![] };
    let mut as_mine = true;

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::field => {
                let mut new_rule = TicketRule { name: "".to_string(), ranges: vec![]};
                for part in line.into_inner() {
                    match part.as_rule() {
                        Rule::fieldname => { new_rule.name = part.as_str().to_string(); },
                        Rule::range => {
                            let mut new_range = Range { min: 0, max: 0 };
                            for minmax in part.into_inner() {
                                match minmax.as_rule() {
                                    Rule::rangemin => {
                                        new_range.min = minmax.as_str().parse().unwrap();
                                    },
                                    Rule::rangemax => {
                                        new_range.max = minmax.as_str().parse().unwrap();
                                    },
                                    _ => {},
                                }
                            }
                            new_rule.ranges.push(new_range);
                        }
                        _ => {},
                    } 
                }
                rules.push(new_rule);
            },
            Rule::yourstag => { as_mine = true; },
            Rule::nearbytag => { as_mine = false; },
            Rule::ticket => {
                let mut ticket = Ticket{ values: vec![] };
                for value in line.into_inner() {
                    if value.as_rule() == Rule::fieldval {
                        ticket.values.push(value.as_str().parse().unwrap());
                    }
                }

                if as_mine {
                    my_ticket = ticket;
                } else {
                    tickets.push(ticket);
                }
            },
            _ => {},
        }
    }

    println!("Read {} rules", rules.len());
    println!("Read {} tickets.", tickets.len());
    println!("My ticket is {:?}", my_ticket);

    let mut error_rate = 0;
    let mut failed_tickets = 0;
    for ticket in tickets {
        let ticket_err = ticket.error_rate(&rules);
        error_rate += ticket_err;
        if ticket_err > 0 {
            failed_tickets += 1;
        }
    }
    println!("Total error rate from {} bad tickets is: {}", failed_tickets, error_rate);
}


#[derive(Parser)]
#[grammar = "parsers/day16.pest"]
pub struct DParser {}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
