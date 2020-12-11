use std::collections::HashMap;
use std::time::SystemTime;
use std::convert::TryInto;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data11.txt")
}

pub fn run() {
    print_day(11);

    let start = SystemTime::now();

    // Let's do this...
    let (rounds, occupied) = run_day(data(), &NeighbourMode::Adjacent);
    println!("For adjacent neighbours, stabilised after {} rounds, {} seats occupied.", rounds, fmt_bright(&occupied));
    let (rounds2, occupied2) = run_day(data(), &NeighbourMode::Sight);
    println!("For line of sight neighbour, stabilised after {} rounds, {} seats occupied.", rounds2, fmt_bright(&occupied2));

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

enum NeighbourMode {
    Adjacent,
    Sight
}

fn run_day(plan: &str, mode: &NeighbourMode) -> (i32, i32) {
    // Load the data
    let mut floor_now: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut seats_only: HashMap<(i32, i32), Tile> = HashMap::new();

    for (i, line) in plan.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let ix: i32 = i.try_into().unwrap();
            let jx: i32 = j.try_into().unwrap();
            match c {
                '#' => {
                    floor_now.insert((ix, jx), Tile::FullSeat);
                    seats_only.insert((ix, jx), Tile::FullSeat);
                },
                'L' => {
                    floor_now.insert((ix, jx), Tile::EmptySeat);
                    seats_only.insert((ix, jx), Tile::EmptySeat);
                },
                _ => {
                    floor_now.insert((ix, jx), Tile::Floor);
                },
            };
        }
    }


    // Work out the 'neighbours' for each element.
    let (neighbours, sensitivity) = match mode {
        NeighbourMode::Adjacent => (neighbours_adjacent(&floor_now), 4),
        NeighbourMode::Sight => (neighbours_sight(&floor_now), 5),
    };

    let mut round = 1;
    loop {
        let (next, changed, occupied) = next_day(&seats_only, &neighbours, sensitivity);
        if changed == 0 {
            break (round, occupied)
        }
        round += 1;
        seats_only = next;
    }
}
fn neighbours_sight(plan: &HashMap<(i32, i32), Tile>) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    neighbours_internal(plan, true)
}

fn neighbours_adjacent(plan: &HashMap<(i32, i32), Tile>) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    neighbours_internal(plan, false)
}

fn neighbours_internal(plan: &HashMap<(i32, i32), Tile>, follow_sight: bool) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut neighbour_map = HashMap::new();
    let directions = [(0, -1), (0, 1), (1, -1), (1, 0), (1, 1), (-1, -1), (-1, 0), (-1, 1)];

    for (i, j) in plan.keys() {
        let mut neighbours = vec![];
        for (di, dj) in &directions {
            let mut try_i = i + di;
            let mut try_j = j + dj;
            loop {
                let maybe_neighbour = plan.get(&(try_i, try_j));
                // If none - we're outside the room. Nothing more to see.
                if maybe_neighbour == None {
                    break;
                }

                // Not floor - this is the neighbour we're interested in.
                if maybe_neighbour != Some(&Tile::Floor) {
                    neighbours.push((try_i, try_j));
                    break;
                }

                // We're not following sight lines.
                if !follow_sight {
                    break;
                }

                // We look through the empty floor.
                try_i += di;
                try_j += dj;
            }
        }
        neighbour_map.insert((*i, *j), neighbours);
    }

    neighbour_map
}

fn next_day(now: &HashMap<(i32, i32), Tile>, neighbours: &HashMap<(i32, i32), Vec<(i32, i32)>>, sensitivity: i32) -> (HashMap<(i32, i32), Tile>, i32, i32) {
    let mut next: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut changed = 0;
    let mut occupied = 0;

    for ((i, j), tile) in now {
        let mut count = 0;
        if *tile == Tile::Floor {
            // Floor stays floor
            next.insert((*i, *j), Tile::Floor);
        } else {
            // Count occupied neighbours
            if let Some(tile_neighbours) = neighbours.get(&(*i, *j)) {
                for neighbour in tile_neighbours {
                    if let Some(Tile::FullSeat) = now.get(neighbour)  {
                        count += 1;
                    }
                }
            }

            if count == 0 {
                next.insert((*i, *j), Tile::FullSeat);
                occupied += 1;
                if *tile != Tile::FullSeat {
                    changed += 1;
                }
            } else if count >= sensitivity {
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
    fn test_small() {
        let data_small = include_str!("../data/data11_small.txt");
        assert_eq!((6, 37), run_day(&data_small, &NeighbourMode::Adjacent));
        assert_eq!((7, 26), run_day(&data_small, &NeighbourMode::Sight));
    }

    #[test]
    #[ignore] 
    fn test_full() {
        assert_eq!((84, 2344), run_day(data(), &NeighbourMode::Adjacent));
        assert_eq!((87, 2076), run_day(data(), &NeighbourMode::Sight));
    }
}
