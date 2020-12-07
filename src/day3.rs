use std::error::Error;
use std::iter::Iterator;
use std::time::SystemTime;

use advent2020::{print_day, print_duration, fmt_bright};

pub fn run() {
    print_day(3);
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data3.txt");
    let contents = String::from_utf8_lossy(cbytes);

    let trajs: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut runs: Vec<usize> = Vec::new();

    for (x, y) in trajs {
        let res = run_xy_string(&contents, x, y).unwrap_or(0);
        if (x, y) == (3, 1) {
            println!("  --> (my first try) {}", fmt_bright(&res));
        }
        runs.push(res);
    }
    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Grand product {}", fmt_bright(&runs.iter().product::<usize>()));
    print_duration(timed);
}

pub fn run_xy_string(contents: &str, x: usize, y: usize) -> Result<usize, Box<dyn Error>> {
    let mut trees = 0;
    for (ypos, line) in contents.lines().enumerate() {
        if (ypos % y) == 0 {
            let chars: Vec<_> = line.chars().collect();
            let c = chars.get((x * ypos / y) % chars.len()).unwrap();
            if *c == '#' {
                trees += 1;
            }
        }
    }

    println!("Going {}, {}, hit {} trees \u{1F61E}", x, y, trees);

    Ok(trees)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_test() {
        let cbytes = include_bytes!("../data/data3.txt");
        let contents = String::from_utf8_lossy(cbytes);
        assert_eq!(Some(68), run_xy_string(&contents, 1, 1).ok());
        assert_eq!(Some(203), run_xy_string(&contents, 3, 1).ok());
        assert_eq!(Some(78), run_xy_string(&contents, 5, 1).ok());
        assert_eq!(Some(77), run_xy_string(&contents, 7, 1).ok());
        assert_eq!(Some(40), run_xy_string(&contents, 1, 2).ok());
    }
}
