use std::collections::HashMap;
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data11.txt")
}

fn data_small() -> &'static str {
    include_str!("../data/data11_small.txt")
}

pub fn run() {
    print_day(11);

    let start = SystemTime::now();

    // Let's do this...
    run_day(data_small());
    run_day(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn run_day(plan: &str) {
    // Load the data
    let mut floor_now = HashMap::new();

    for (i, line) in plan.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'L' => floor_now.insert((i, j), Tile::EmptySeat),
                '#' => floor_now.insert((i, j), Tile::FullSeat),
                '.' => floor_now.insert((i, j), Tile::Floor),
                 _ => floor_now.insert((i, j), Tile::Floor),
            };
        }
    }

    let mut round = 1;
    loop {
        let (next, changed, occupied) = next_day(&floor_now);
        println!("[{}] {} occupied seats. (Change = {})", round, occupied, changed);
        if changed == 0 {
            break;
        }
        round += 1;
        floor_now = next;
    }
}

fn next_day(now: &HashMap<(usize, usize), Tile>) -> (HashMap<(usize, usize), Tile>, usize, usize) {
    let mut next: HashMap<(usize, usize), Tile> = HashMap::new();
    let mut changed = 0;
    let mut occupied = 0;

    for ((i, j), tile) in now {
        let mut count = 0;
        if *tile == Tile::Floor {
            // Floor stays floor
            next.insert((*i, *j), Tile::Floor);
        } else {
            // Count occupied neighbours
            if *i > 0 {
                if *j > 0 {
                    if let Some(Tile::FullSeat) = now.get(&(*i -1, *j - 1)) {
                        count += 1;
                    }
                }
                
                if let Some(Tile::FullSeat) = now.get(&(*i - 1, *j)) {
                    count += 1;
                }
                
                if let Some(Tile::FullSeat) = now.get(&(*i - 1, *j + 1)) {
                    count += 1;
                }
            }

            if *j > 0 {
                if let Some(Tile::FullSeat) = now.get(&(*i, *j - 1)) {
                    count += 1;
                }

                if let Some(Tile::FullSeat) = now.get(&(*i + 1, *j - 1)) {
                    count += 1;
                }
            }

            if let Some(Tile::FullSeat) = now.get(&(*i + 1, *j)) {
                count += 1;
            }

            if let Some(Tile::FullSeat) = now.get(&(*i, *j + 1)) {
                count += 1;
            }


                if let Some(Tile::FullSeat) = now.get(&(*i + 1, *j + 1)) {
                    count += 1;
                }

            if count == 0 {
                next.insert((*i, *j), Tile::FullSeat);
                occupied += 1;
                if *tile != Tile::FullSeat {
                    changed += 1;
                }
            } else if count >= 4 {
                next.insert((*i, *j), Tile::EmptySeat);
                if *tile != Tile::EmptySeat {
                    changed += 1;
                }
            } else {
                next.insert((*i, *j), *tile);
                if *tile == Tile::FullSeat {
                    occupied += 1;
                }
            }
        }
    }

    (next, changed, occupied)
}

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Floor,
    EmptySeat,
    FullSeat,
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
