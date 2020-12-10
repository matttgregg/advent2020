use std::time::SystemTime;
use std::collections::HashMap;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data10.txt")
}

pub fn run() {
    print_day(10);
    let start = SystemTime::now();

    let (product, target, chains) = check_jolts(data());
    println!("Target is {}, reached via max jump sequence {}. There are {} valid sequences in total.", target,
             fmt_bright(&product), fmt_bright(&chains));

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn check_jolts(data: &str) -> (usize, usize, usize) {
    let mut adapters: Vec<usize> = data.lines().filter_map(|x| x.parse::<usize>().ok()).collect();
    adapters.sort_unstable();

    let mut single_jumps = 0;
    let mut triple_jumps = 1; // Always get one for the jump to my device.
    let mut jolts = 0; // The zero powered socket.

    for adapter in &adapters {
        if adapter - jolts == 1 {
            single_jumps += 1;
        } else if adapter - jolts == 3 {
            triple_jumps += 1;
        }
        jolts = *adapter;
    }

    let target = jolts + 3;
    let mut memo = HashMap::new();
    let chains = valid_chains(&adapters, 0, 0, target, &mut memo);
    (single_jumps * triple_jumps, target, chains)
}

fn valid_chains(adapters: &[usize], idx: usize, curr: usize, target: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    // We count the valid chains from here.
    if let Some(val) = memo.get(&(idx, curr)) {
        return *val;
    }

    let mut valid = 0;
    if (target - curr) <= 3 {
        // We already there! This is a valid target.
        valid += 1;
    }

    // If the next value is within 3, try that.
    if let Some(jolt) = adapters.get(idx) {
        // If within 3, we can use it.
        if (jolt - curr) <= 3 {
            valid += valid_chains(adapters, idx + 1, *jolt, target, memo);
            // Also, try skipping it
            valid += valid_chains(adapters, idx + 1, curr, target, memo);
        }
    }
    memo.insert((idx, curr), valid);
    
    // Note that if we're out of index, or the gap is too big, we don't recurse.
    valid
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small() {
        let small_data = "16
10
15
5
1
11
7
19
6
12
4";

        assert_eq!((35,22,8), check_jolts(small_data));
    }
        
    #[test]
    fn test_med() {
        let med_data = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        assert_eq!((220, 52, 19208), check_jolts(med_data));
    }

    #[test]
    fn test_all() {
        assert_eq!((1917, 152, 113387824750592), check_jolts(data()));
    }
}
