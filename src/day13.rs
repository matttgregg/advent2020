use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data13.txt")
}

pub fn run() {
    print_day(13);

    let start = SystemTime::now();

    // Let's do this...
    let waited = waiting_times(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("My waiting number is {}", fmt_bright(&waited));

    print_duration(timed);
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
