use std::time::SystemTime;
use std::collections::HashSet;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data24.txt")
}

pub fn run() {
    print_day(24);

    let start = SystemTime::now();

    let data_small = include_str!("../data/data24_small.txt");
    parse_tiles(&data_small);
    parse_tiles(data());

    // Let's do this...

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn parse_tiles(data: &str) {
    // Each line is non-delimited e/se/ne/w/nw/ne
    // We pick our coordinate system s.t. ne/sw is on diagonal, nw, se on constant.
    let mut black_tiles = HashSet::new();

    for line in data.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut x = 0;
        let mut y = 0;
        let mut previous = None;

        for c in line.chars() {
            match c {
                's' | 'n' => {
                    if let Some(p) = previous {
                        panic!(format!("Unexpect adjoining chars : {} {}", p, c));
                    } else {
                        previous = Some(c); // We stash the n/s to resolve later.
                    }
                },
                'e' => {
                    match previous {
                        None => {
                            x += 1;
                        },
                        Some('n') => {
                            y += 1; // ne is on the daigonal.
                            x += 1;
                        },
                        Some('s') => {
                            y -= 1; // se is on the vertical.
                        },
                        _ => panic!("unexpected direction."),
                    };
                    previous = None;
                },
                'w' => {
                    match previous {
                        None => {
                            x -= 1;
                        },
                        Some('n') => {
                            y += 1; // nw is on the vertical.
                        },
                        Some('s') => {
                            y -= 1;
                            x -= 1; // sw is on the diagonal.
                        },
                        _ => panic!("unexpected direction"),
                    };
                    previous = None;
                },
                _ => panic!("unexpected direction"),
            };
        }

        if previous != None {
            panic!("Unresolved char at end of line.");
        }

        if black_tiles.contains(&(x, y)) {
            println!("Unflipping tile {}, {}", x, y);
            black_tiles.remove(&(x, y));
        } else {
            println!("Flipping tile {}, {}", x, y);
            black_tiles.insert((x, y));
        }
    }

    println!("Total black tiles: {}", black_tiles.len());
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
