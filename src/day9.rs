use std::collections::HashSet;
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data9.txt")
}

fn data_test() -> &'static str {
    "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
}

pub fn run() {
    print_day(9);

    let start = SystemTime::now();

    // Let's do this...
    let (p1, p2) = verify_data(data(), 25);
    println!("XMAS invalid value: {}", fmt_bright(&p1));
    println!("XMAS weakeness detected: {}", fmt_bright(&p2));

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn verify_data(data: &str, window :usize) -> (i64, i64) {
    let vals: Vec<i64> = data.lines().filter_map(|x| x.parse::<i64>().ok()).collect();
    let mut checking = HashSet::new();
    let mut invalid = 0;

    for (i, val) in vals.iter().enumerate() {
        if i < window {
            checking.insert(val);
        } else {
            // Need to check previous 25 for sum.
            let mut found = false;
            for c in &vals[(i - window)..i] {
                if ((2 * c) as i64 != *val) && checking.contains(&(val - c)) {
                    found = true;
                    break;
                }
            }
            if !found {
                invalid = *val;
                break;
            }

            // Otherwise we need to rotate our checking set.
            checking.remove(&vals[i - window]);
            checking.insert(val);
        }
    }

    // Now search for range which sums to this.
    for (i, _lower) in vals.iter().enumerate() {
        let mut upper = i + 1;
        while vals[i..upper].iter().sum::<i64>() < invalid {
            upper += 1;
        }

        if vals[i..upper].iter().sum::<i64>() == invalid {

            // Now search for min/max values
            let max = vals[i..upper].iter().max().unwrap_or(&0);
            let min = vals[i..upper].iter().min().unwrap_or(&0);
            return (invalid, min + max);
        }

    }

    (invalid, 0)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
