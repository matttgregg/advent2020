use std::time::SystemTime;
use std::convert::TryInto;

use advent2020::{fmt_bright, print_day, print_duration};

pub fn run() {
    print_day(15);

    let data = vec![11,0,1,10,5,19];

    let start = SystemTime::now();

    // Let's do this...
    let res1 = repeat_to(&data, 2020);
    let res2 = repeat_to(&data, 30_000_000);

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("After waiting his turn, the final elf screams {}", fmt_bright(&res1));
    println!("After waiting even longer, the final elf screams {}, and his hat falls off.", fmt_bright(&res2));
    print_duration(timed);
}

fn repeat_to(starting: &[u32], target: u32) -> u32 {
    let mut last = 0_u32;
    let mut cache = vec![u32::MAX; 30_000_000];

    for i in 0..target {
        let next;
        if i < starting.len().try_into().unwrap() {
            next = starting[i as usize];
        } else if cache[last as usize] == u32::MAX {
            next = 0;
        } else {
            next = i - cache[last as usize];
        }

        // Make a note of the last number.
        cache[last as usize] = i;
        last = next;
    }
    last 
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_2020() {
        let data = vec![11,0,1,10,5,19];
        assert_eq!(870, repeat_to(&data, 2020));
    }

    #[test]
    fn test_30mill() {
        let data = vec![11,0,1,10,5,19];
        assert_eq!(9136, repeat_to(&data, 30_000_000));
    }
}
