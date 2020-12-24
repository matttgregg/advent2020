use std::time::SystemTime;
use std::collections::{HashSet, HashMap};

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data24.txt")
}

pub fn run() {
    print_day(24);

    let start = SystemTime::now();

    // Let's do this...
    let (initial, final_tiles) = parse_tiles(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("The workman initially set out {} black tiles.", fmt_bright(&initial));
    println!("After toiling for 100 hours, there are {} black tiles.", fmt_bright(&final_tiles));
    print_duration(timed);
}

fn parse_tiles(data: &str) -> (usize, usize) {
    // Each line is non-delimited e/se/ne/w/nw/ne
    // We pick our coordinate system s.t. ne/sw is on diagonal, nw, se on constant.
    let mut black_tiles = HashSet::new();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        let mut x = 0_i64;
        let mut y = 0_i64;
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
                        Some(_) => panic!("unexpected direction."),
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
                        Some(_) => panic!("unexpected direction"),
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
            black_tiles.remove(&(x, y));
        } else {
            black_tiles.insert((x, y));
        }
    }

    let initial_tiles = black_tiles.len();

    // Perform 100 iterations.

    for _ in 0..100 {
        black_tiles = flip_tiles(&black_tiles);
    }

    (initial_tiles, black_tiles.len())
}

fn flip_tiles(tiles: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    // Build the adjacency lists:
    let mut marked_neighbours: HashMap<(i64, i64), i64> = HashMap::new();

    for (x, y) in tiles {
        *marked_neighbours.entry((x + 1, *y)).or_insert(0) += 1;
        *marked_neighbours.entry((x - 1, *y)).or_insert(0) += 1;
        *marked_neighbours.entry((x + 1, y + 1)).or_insert(0) += 1;
        *marked_neighbours.entry((x - 1, y - 1)).or_insert(0) += 1;
        *marked_neighbours.entry((*x, y + 1)).or_insert(0) += 1;
        *marked_neighbours.entry((*x, y - 1)).or_insert(0) += 1;
    }

    let mut new_tiles = HashSet::new();

    for (tile, neighbour_count) in marked_neighbours {
        if tiles.contains(&tile) {
            if (1..=2).contains(&neighbour_count) {
            // Black tiles with 1 or 2 neighbours stay black.
            new_tiles.insert(tile);
            }
        } else if neighbour_count == 2 {
            // White tiles with exactly 2 marked neighbours turn black.
            new_tiles.insert(tile);
        }
    }

    new_tiles

}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small() {
        let data_small = include_str!("../data/data24_small.txt");
        assert_eq!((10, 2208), parse_tiles(&data_small));
    }

    #[test]
    fn test_all() {
        assert_eq!((377, 4231), parse_tiles(data()));
    }
}
