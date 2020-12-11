use std::convert::TryInto;
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data11.txt")
}

pub fn run() {
    print_day(11);

    let start = SystemTime::now();

    // Let's do this...
    let (rounds, occupied) = run_day(data(), &NeighbourMode::Adjacent);
    println!(
        "For adjacent neighbours, stabilised after {} rounds, {} seats occupied.",
        rounds,
        fmt_bright(&occupied)
    );
    let (rounds2, occupied2) = run_day(data(), &NeighbourMode::Sight);
    println!(
        "For line of sight neighbour, stabilised after {} rounds, {} seats occupied.",
        rounds2,
        fmt_bright(&occupied2)
    );

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

enum NeighbourMode {
    Adjacent,
    Sight,
}

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Floor,
    EmptySeat,
    FullSeat,
}

fn run_day(plan: &str, mode: &NeighbourMode) -> (i32, i32) {
    // Load the data
    let lines = plan.lines();
    let mut floor_now: Vec<Vec<Tile>> = Vec::with_capacity(plan.lines().count());
    for l in lines {
        floor_now.push(vec![Tile::Floor; l.len()]);
    }
    let mut seats_only: Vec<(usize, usize)> = vec![];

    for (i, line) in plan.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    floor_now[i][j] = Tile::FullSeat;
                    seats_only.push((i, j));
                }
                'L' => {
                    floor_now[i][j] = Tile::EmptySeat;
                    seats_only.push((i, j));
                }
                _ => {
                    floor_now[i][j] = Tile::Floor;
                }
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
        let (next, changed, occupied) = next_day(&seats_only, &floor_now, &neighbours, sensitivity);
        if changed == 0 {
            break (round, occupied);
        }
        round += 1;
        floor_now = next;
    }
}
fn neighbours_sight(plan: &[Vec<Tile>]) -> Vec<Vec<Vec<(usize, usize)>>> {
    neighbours_internal(plan, true)
}

fn neighbours_adjacent(plan: &[Vec<Tile>]) -> Vec<Vec<Vec<(usize, usize)>>> {
    neighbours_internal(plan, false)
}

fn neighbours_internal(plan: &[Vec<Tile>], follow_sight: bool) -> Vec<Vec<Vec<(usize, usize)>>> {
    let mut neighbour_map: Vec<Vec<Vec<(usize, usize)>>> = Vec::with_capacity(plan.len());
    let directions: Vec<(i32, i32)> = vec![
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    for (i, row) in plan.iter().enumerate() {
        neighbour_map.push(vec![Vec::with_capacity(8); row.len()]);
        for (j, v) in row.iter().enumerate() {
            if *v == Tile::Floor {
                continue; // We don't need neighbours for the floor.
            }
            neighbour_map[i][j] = Vec::with_capacity(8);
            for (di, dj) in &directions {
                let mut try_us_i: i32 = i.try_into().unwrap();
                try_us_i += di;
                let mut try_us_j: i32 = j.try_into().unwrap();
                try_us_j += dj;
                loop {
                    let try_i: Option<usize> = try_us_i.try_into().ok();
                    let try_j: Option<usize> = try_us_j.try_into().ok();

                    if try_i == None || try_j == None {
                        // Out of bounds.
                        break;
                    }

                    let maybe_row = plan.get(try_i.unwrap());

                    if maybe_row == None {
                        break;
                    }

                    let maybe_neighbour = maybe_row.unwrap().get(try_j.unwrap());
                    // If none - we're outside the room. Nothing more to see.
                    if maybe_neighbour == None {
                        break;
                    }

                    // N ot floor - this is the neighbour we're interested in.
                    if maybe_neighbour != Some(&Tile::Floor) {
                        neighbour_map[i][j].push((try_i.unwrap(), try_j.unwrap()));
                        break;
                    }

                    // We're not following sight lines.
                    if !follow_sight {
                        break;
                    }

                    // We look through the empty floor.
                    try_us_i += di;
                    try_us_j += dj;
                }
            }
        }
    }

    neighbour_map
}

fn next_day(
    seats: &[(usize, usize)],
    now: &[Vec<Tile>],
    neighbours: &[Vec<Vec<(usize, usize)>>],
    sensitivity: i32,
) -> (Vec<Vec<Tile>>, i32, i32) {
    let mut next: Vec<Vec<Tile>> = Vec::with_capacity(now.len());
    let mut changed = 0;
    let mut occupied = 0;

    for (i, j) in seats {
        while next.len() <= *i {
            next.push(vec![Tile::Floor; *j])
        }
        while next[*i].len() <= *j {
            next[*i].push(Tile::Floor);
        }

        let mut count = 0;
        let tile = now[*i][*j];
            // Count occupied neighbours
                for (ni, nj) in &neighbours[*i][*j] {
                    if now[*ni][*nj] == Tile::FullSeat {
                        count += 1;
                    }
                }

            if count == 0 {
                next[*i][*j] = Tile::FullSeat;
                occupied += 1;
                if tile != Tile::FullSeat {
                    changed += 1;
                }
            } else if count >= sensitivity {
                next[*i][*j] = Tile::EmptySeat;
                if tile != Tile::EmptySeat {
                    changed += 1;
                }
            } else {
                next[*i][*j] = tile;
                if tile == Tile::FullSeat {
                    occupied += 1;
                }
            }
    }

    (next, changed, occupied)
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
