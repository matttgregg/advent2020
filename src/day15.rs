use std::time::SystemTime;
use std::collections::HashMap;

use advent2020::{fmt_bright, print_day, print_duration};

pub fn run() {
    print_day(15);

    let data = vec![11,0,1,10,5,19];

    let start = SystemTime::now();

    // Let's do this...
    //repeat_to(&[0,3,6], 2020);
    let res1 = repeat_to(&data, 2020);
    let res2 = repeat_to(&data, 30_000_000);

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("After waiting his turn, the final elf screams {}", fmt_bright(&res1));
    println!("After waiting even longer, the final elf screams {}, and his hat falls off.", fmt_bright(&res2));
    print_duration(timed);
}

fn repeat_to(starting: &[u64], target: usize) -> u64 {
    let mut seen = HashMap::new();
    let mut last = 0;

    for i in 0..target {
        let next;
        if i < starting.len() {
            next = starting[i];
        } else if let Some(j) = seen.get(&last) {
            // Have we seen the last number before?
            next = (i - j) as u64;
        } else {
            next = 0;
        }

        // Make a note of the last number.
        seen.insert(last, i);
        last = next;
    }
    last 
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
