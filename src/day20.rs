use std::time::SystemTime;
use std::collections::HashMap;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data20.txt")
}

pub fn run() {
    print_day(20);

    let start = SystemTime::now();

    // Let's do this...
    let data_small = include_str!("../data/data20_small.txt");
    parse_tiles(&data_small);
    parse_tiles(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn parse_tiles(data: &str) {
    let mut tiles = vec![];
    let mut scan = vec![];
    let mut tile_index = 0;
    for line in data.lines() {
        if line.starts_with("Tile ") {
            // We know the line always looks like: "Tile xxxx:", so id from 5-8.
            let idx = &line[5..9];
            tile_index = idx.parse().unwrap();
            scan = vec![];
        } else if !line.is_empty() {
            // This should be a scan line:
            let mut scan_line = vec![];
            for c in line.chars() {
                match c {
                    '.' => { scan_line.push(0); },
                    '#' => { scan_line.push(1); },
                    _ => {}, // Silently ignore unexpected chars.
                }
            }
            scan.push(scan_line);
            if scan.len() == 10 {
                // Have completed a tile.
                tiles.push(Tile::new(tile_index, scan));
                scan = vec![];
            }
        }
    }

    println!("Read {} tiles:", tiles.len());

    for tile in &tiles {
        println!("Tile: {} Keys: {}/{}|{}/{}|{}/{}|{}/{}", tile.index,
                 tile.key(0, 0), tile.key(0, 2),
                 tile.key(1, 0), tile.key(1, 2),
                 tile.key(2, 0), tile.key(2, 2),
                 tile.key(3, 0), tile.key(3, 2)
        );
    }

    // Store the key/edge/orientation triples for every number.
    let mut lookup: HashMap<u16, Vec<(u16, u8, bool)>> = HashMap::new();
    for tile in &tiles {
        for e in 0..4 {
            let k_natural = tile.key(e, 0);
            lookup.entry(k_natural).or_insert_with(|| vec![]).push((tile.index, e, false));

            let k_twisted = tile.key(e, 2);
            lookup.entry(k_twisted).or_insert_with(|| vec![]).push((tile.index, e, true));
        }
    }

    let mut corners: Vec<u64> = vec![];

    // Now lets look at all the tiles, and see which have mathcing edges:
    for tile in &tiles {
        println!("Tile: {}:", tile.index);
        let mut matching = 0;
        for e in 0..4 {
            if let Some(matches) = lookup.get(&tile.key(e, 0)) {
                if matches.len() > 1 {
                    matching += 1;
                    for (m_tile, m_edge, m_twist) in matches {
                        if *m_tile != tile.index {
                            println!("-> E:{} matches tile {}, edge {}, twisted?{}", e, m_tile, m_edge, m_twist);
                        }
                    }
                }
            }
        }
        println!("Tile: {} matches {} edges.", tile.index, matching);
        if matching  == 1 {
            panic!("Tile doesn't fit!");
        }
        if matching == 2 {
            corners.push(tile.index as u64);
        }
    }

    println!("Found potential corners {:?} . Prod = {}", corners, corners.iter().product::<u64>());
}

struct Tile {
    index: u16,
    orientation: u8,
    scans: Vec<Vec<u8>>
}

impl Tile {
    fn new(idx: u16, scans: Vec<Vec<u8>>) -> Self {
        Tile { index: idx, orientation: 0, scans }
    }

    fn edge_orientation(edge: u8) -> bool {
        match edge {
            0 | 1 => true,
            2 | 3 => false,
            _ => panic!("Unexpected direction."),
        }
    }

    fn key_from_vec(vals: &[u8]) -> u16 {
        let mut key = 0;
        for (i, v) in vals.iter().enumerate() {
            if *v > 0 {
                key += 2_u16.pow(i as u32);
            }
        }
        key
    }

    fn key(&self, side: u8, shift: u8) -> u16 {
        let shifted = (side + shift) % 4;
        
        let mut key_vec = self.key_vec(shifted);
        if Self::edge_orientation(side) != Self::edge_orientation(shifted) {
            key_vec.reverse();
        }
        
        Self::key_from_vec(&key_vec)
    }

    fn key_vec(&self, side: u8) -> Vec<u8> {
        match side {
            0 => self.scans[0].to_vec(),
            1 => self.scans.iter().map(|x| *x.last().unwrap()).collect(),
            2 => self.scans.last().unwrap().to_vec(),
            3 => self.scans.iter().map(|x| *x.first().unwrap()).collect(),
            _ => panic!("Unexpected direction!")
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
