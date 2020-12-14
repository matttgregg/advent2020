use std::time::SystemTime;
use std::collections::HashMap;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data13.txt")
}

pub fn run() {
    print_day(13);

    let start = SystemTime::now();

    // Let's do this...
    let waited = waiting_times(data());
    let competition_solution = solve_competition(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("My waiting number is {}", fmt_bright(&waited));
    println!("Found a bus alignment at {}", fmt_bright(&competition_solution));

    print_duration(timed);
}

fn solve_competition(timetable: &str) -> u64 {
    let mut data = timetable.lines();
    data.next(); // Discard the start time for the competition.
    
    let mut bus_data = data.next().unwrap().split(',');
    // Take the first bus.
    let first_bus = bus_data.next().unwrap().parse::<u64>().unwrap();

    let mut offset_buses: Vec<(u64, u64)> = vec![];
    for (i, maybe_bus) in bus_data.enumerate() {
        if let Ok(bus) = maybe_bus.parse::<u64>() {
            offset_buses.push((bus, (i + 1) as u64));
        }
    }

    let mut t = 0;
    let mut iters = 0;
    let mut delta = first_bus;
    let mut alignment: HashMap<u64, u64> = HashMap::new(); // Track alignments.
    loop {
        let mut matched = true;
        for (bus, offset) in &offset_buses {
            if (t + offset) % bus == 0  {
                // This bus *did* fit. Lock in this alignment.
                if !alignment.contains_key(bus) {
                    alignment.insert(*bus, t);
                    delta *= bus;
                    println!("Aligned with {} after {} iterations. (New delta: {})", bus, iters, delta);
                }
            } else {
                // This bus didn't fit. Stop checking and move on.
                matched = false;
            }
        }

        if matched {
            println!("Found a solution after {} iterations.", iters);
            return t;
        }
        iters += 1;
        t += delta;
    }
}

fn waiting_times(timetable: &str) -> u64 {
    let mut data = timetable.lines();
    let start = data.next().unwrap().parse::<u64>().unwrap();
    let buses: Vec<u64> = data.next().unwrap().split(',').filter_map(|x| x.parse::<u64>().ok()).collect();

    let mut t = start;
    loop {
        // Check all the buses at the current time.
        for bus in &buses {
            if t % bus == 0 {
                println!("[{}] After waiting {}, bus {} is here!", t, t - start, bus);
                return (t - start) * bus;
            }
        }
        t += 1;
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
