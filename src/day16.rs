use pest::Parser;
use pest_derive::Parser;

use std::collections::HashMap;
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data16.txt")
}

pub fn run() {
    print_day(16);

    let start = SystemTime::now();

    let (res1, res2) = parse_tickets(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Error rate across all tickets is {}, my departure product is {} .", fmt_bright(&res1), fmt_bright(&res2));
    print_duration(timed);
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u64>
}

impl Ticket {
    fn error_rate(&self, rules: &[TicketRule]) -> Option<u64> {
        let mut rate = 0;
        let mut valid = true;
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
                valid = false;
            }
        }

        if valid {
            None
        } else {
            Some(rate)
        }
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

fn import_tickets(data: &str) -> (Ticket, Vec<Ticket>, Vec<TicketRule>) {
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

    (my_ticket, tickets, rules)
}

fn parse_tickets(data: &str) -> (u64, u64) {

    let (my_ticket, tickets, rules) = import_tickets(data);

    let mut error_rate = 0;
    let mut good_tickets = vec![];
    for ticket in tickets {
        if let Some(ticket_err) = ticket.error_rate(&rules) {
            error_rate += ticket_err;
        } else {
            good_tickets.push(ticket);
        }
    }
    let field_count = my_ticket.values.len();

    let mut fields = HashMap::new();
    let mut can_be: HashMap<&String, Vec<usize>> = HashMap::new();

    // Initialise fields map
    for rule in &rules {
        can_be.insert(&rule.name, vec![]);
    }

    // Get the *good* tickets.
    for i in 0..field_count {
        let mut maybes = vec![];
        let mut all_values = vec![];
        for ticket in &good_tickets {
            all_values.push(ticket.values[i]);
        }
        // Now - which rules match all values?
        for rule in &rules {
            let mut can_match = true;
            for val in &all_values {
                if !rule.fits_any(*val) {
                    can_match = false;
                    break;
                }
            }
            if can_match {
                maybes.push(&rule.name);
                can_be.get_mut(&rule.name).unwrap().push(i); 
            }
        }
        fields.insert(i, maybes);
    }

    let mut matched = HashMap::new();
    let mut matched_indices = HashMap::new();
    // Now we loop, finding uniquely sepcified values.
    loop {
        let mut updates = 0;
        // look over fields
        for (field_name, field_possibilities) in &can_be {
            if matched.contains_key(field_name) {
                // We've already assigned this field.
                continue;
            }

            // What *could* this field be?
            let mut possibilities = vec![];
            for p in field_possibilities {
                if !matched_indices.contains_key(p) {
                    possibilities.push(*p);
                }
            }
            // If there's exactly one possibility, use it!
            if possibilities.len() == 1 {
                let found = possibilities.get(0).unwrap();
                matched_indices.insert(*found, field_name);
                matched.insert(field_name, *found);
                updates += 1;
            }
        }
        if updates == 0 {
            // Nothing more to be done.
            break;
        }
    }

    let mut departure_product = 1;
    for (k, v) in matched {
        if k.starts_with("departure") {
            let val = my_ticket.values.get(v).unwrap();
            departure_product *= val;
            println!("Field {} -> {}", v, k);
        }
    }

    (error_rate, departure_product)
}


#[derive(Parser)]
#[grammar = "parsers/day16.pest"]
pub struct DParser {}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small() {
        let data_small = include_str!("../data/data16_small.txt");
        let data_small2 = include_str!("../data/data16_small2.txt");
        assert_eq!((71,1), parse_tickets(&data_small));
        assert_eq!((0,1), parse_tickets(&data_small2));
    }

    #[test]
    fn test_all() {
        assert_eq!((27850, 491924517533), parse_tickets(data()));
    }
}
