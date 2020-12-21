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
    //parse_tiles(data());

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
    let mut oriented_corners: Vec<Oriented> = vec![];
    let mut tile_map: HashMap<u16, &Tile> = HashMap::new();

    // Now lets look at all the tiles, and see which have mathcing edges:
    for tile in &tiles {
        println!("Tile: {}:", tile.index);
        tile_map.insert(tile.index, tile);
        let mut matching = 0;
        let mut matched_edges = [0;4];
        for e in 0..4 {
            if let Some(matches) = lookup.get(&tile.key(e, 0)) {
                if matches.len() > 1 {
                    matching += 1;
                    matched_edges[e as usize] = 1;
                    for (m_tile, m_edge, m_twist) in matches {
                        if *m_tile != tile.index {
                            println!("-> E:{} matches tile {}, edge {}, twisted?{}", e, m_tile, m_edge, m_twist);
                        }
                    }
                }
            }
        }
        println!("Tile: {} matches {} edges: {:?}.", tile.index, matching, matched_edges);
        if matching  == 1 {
            panic!("Tile doesn't fit!");
        }
        if matching == 2 {
            corners.push(tile.index as u64);

            let orientation = match matched_edges {
                [0, 1, 1, 0] => 0,
                [0, 0, 1, 1] => 1,
                [1, 0, 0, 1] => 2,
                [1, 1, 0, 0] => 3,
                _ => panic!("Unexpected corner orientation."),
            };
            oriented_corners.push(Oriented::new(orientation, tile));
        }
    }

    println!("Found potential corners {:?} . Prod = {}", corners, corners.iter().product::<u64>());
    println!("Working from oriented corners: {:?}", oriented_corners);

    // ! Lets build the image array!
    let mut init: Option<&Oriented> = None;
    let mut anchor_tile = oriented_corners.iter().next().unwrap().copy();
    let mut combined: Vec<Vec<Oriented>> = vec![];
    // Loop on rows.
    loop {
        // Find the first tile in the row.
        if init.is_none() {
            // Pull first from one of the corners.
            let first_corner = oriented_corners.iter().next().unwrap();
            init = Some(first_corner);
        } else if let Some(matches) = lookup.get(&anchor_tile.edge_key(2)) {
            let current = anchor_tile.copy();
            println!("Matching bottom edge: {}", current.edge_string(2));
                if let Some((t_index,t_edge, _)) = matches.iter().filter(|(t_index, _, _)| *t_index != current.tile.index).next() {
                    // We've got the tile and edge.
                    let tile = tile_map.get(t_index).unwrap();
                    // Find the orientation and flip.
                    anchor_tile = Oriented::new((t_edge + 1) % 4, tile);
                    if anchor_tile.edge_key(0) != current.edge_key(2) {
                        for orientation in 0..4 {
                            anchor_tile = Oriented::new(orientation, tile);
                            if anchor_tile.edge_key(0) == current.edge_key(2) {
                                break
                            }
                            anchor_tile = anchor_tile.flipped();
                            if anchor_tile.edge_key(0) == current.edge_key(2) {
                                break
                            }
                        }
                    }
                    println!("B ALIGN:{}", current.edge_string(2));
                    println!("T ALIGN:{}", anchor_tile.edge_string(0));
                    init = Some(&anchor_tile);
                } else {
                    // We do not have a matching tile.
                    break;
                }
        }

        let mut current = Oriented::new(anchor_tile.orientation, &anchor_tile.tile);
        let mut row = vec![Oriented::new(anchor_tile.orientation, &anchor_tile.tile)];

        loop {
            // Loop within the row.
            if let Some(matches) = lookup.get(&current.edge_key(1)) {
                if let Some((t_index,t_edge, _)) = matches.iter().filter(|(t_index, _, _)| *t_index != current.tile.index).next() {
                    // We've got the tile and edge.
                    let tile = tile_map.get(t_index).unwrap();
                    // Find the orientation and flip.
                    let mut next_oriented = Oriented::new((t_edge + 1) % 4, tile);
                    if next_oriented.edge_key(3) != current.edge_key(1) {
                        for orientation in 0..4 {
                            next_oriented = Oriented::new(orientation, tile);
                            if next_oriented.edge_key(3) == current.edge_key(1) {
                                println!("Matched:{}:natural", orientation);
                                break
                            }
                            next_oriented = Oriented::new(orientation, &tile.flipped());
                            if next_oriented.edge_key(3) == current.edge_key(1) {
                                println!("Matched:{}:flipped", orientation);
                                break
                            }
                        }
                    }
                    println!("R ALIGN:{}", current.edge_string(1));
                    println!("L ALIGN:{}", next_oriented.edge_string(3));
                    current = next_oriented.copy();
                    row.push(next_oriented);
                } else {
                    // We do not have a matching tile.
                    break;
                }
            } else {
                break;
            }
        }
        combined.push(row);
    }

    let mut chart: Vec<String> = vec![];
    let mut full_chart: Vec<String> = vec![];
    println!("Re-assembled chart:");
    println!();
    for row in combined {
        for ri in 0..10 {
            let mut row_chars = String::from("");
            let mut full_row_chars = String::from("");
            for t in &row {
                if ri == 0 {
                    print!(" {}/{} ", t.tile.index, t.orientation);
                }

                if ri < 8 {
                    row_chars.push_str(&t.row(ri).to_string());
                }

                full_row_chars.push_str(&t.full_row(ri).to_string());
                full_row_chars.push_str(" ");
            }
            if ri < 8 {
                chart.push(row_chars);
            }

            full_chart.push(full_row_chars);
        }
        full_chart.push(String::from(""));
        println!();
    }

    for fr in full_chart {
        println!("{}", fr);
    }

    let row_count = chart.len();
    let col_count = chart.iter().next().unwrap().len();

    let mut hash_count = 0;
    let mut monster_count = 0;
    for ri in 0..row_count {
        for ci in 0..col_count {
            let chart_row: String = chart.get(ri).unwrap().to_string();
            if chart_row.get(ci..ci + 1).unwrap() == "#" {
                hash_count += 1;
            }
            if monster_at(&chart, ri, ci) {
                monster_count += 1;
            }
        }
    }

    for row in chart {
        println!("{}", row);
    }
    println!("Saw {} waves and {} monsters.", hash_count  - 15 * monster_count, monster_count);
 }

//                   # 
// #    ##    ##    ###
//  #  #  #  #  #  #   
fn monster_at(chart: &Vec<String>, row: usize, col: usize) -> bool {
    let monster_coords1: Vec<usize> = vec![18];
    let monster_coords2 = vec![0, 5, 6, 11, 12, 17, 18, 19];
    let monster_coords3 = vec![1, 4, 7, 10, 13, 16];

    let monster_coords = vec![monster_coords1, monster_coords2, monster_coords3];

    // Check the array is big enough.
    if chart.len() <= row + 2 {
        return false;
    }
    
    if chart.iter().next().unwrap().len() <= col + 19 {
        return false;
    }

    // print!("{}, {} Looking for monsters:", row, col);
    for (row_delta, monster_row) in monster_coords.iter().enumerate() {
        let chars = chart.get(row + row_delta).unwrap();
        for monster_col in monster_row {
            let check_col = monster_col + col;
            if chars.get(check_col..check_col + 1).unwrap() != "#" {
      //          println!();
                return false;
            }
     //       print!("{}",row_delta);
        }
    } 

    //println!("Found!");
    return true;
}

#[derive(Debug)]
struct Oriented {
    tile: Tile,
    orientation: u8
}

impl Oriented {
    fn new(orientation: u8, tile: &Tile) -> Self {
        Self{orientation, tile: Tile { index:tile.index, scans: tile.scans.to_owned()}}
    }

    fn copy(&self) -> Self {
        Self::new(self.orientation, &self.tile)
    }

    fn turn(&mut self) {
        self.orientation = (self.orientation + 1) % 4;
    }

    fn edge_key(&self, e: u8) -> u16 {
        self.tile.key(e, self.orientation)
    }

    fn edge_string(&self, e: u8) -> String {
        self.tile.edge_str(e, self.orientation)
    }

    fn flipped(&self) -> Self {
        Self{
            orientation: self.orientation,
            tile: self.tile.flipped()
        }
    }

    fn row(&self, r: usize) -> String {
        let mut row_bits: Vec<u8> = vec![];
        for i in 0..8 {
            row_bits.push(match self.orientation {
                0 => *self.tile.scans.get(r + 1).unwrap().get(i + 1).unwrap(),
                2 => *self.tile.scans.get(9 - r - 1).unwrap().get(i + 1).unwrap(),
                1 => *self.tile.scans.get(i + 1).unwrap().get(9 - r - 1).unwrap(),
                3 => *self.tile.scans.get(i + 1).unwrap().get(r + 1).unwrap(),
                _ => panic!("Unexpected orientation."),
            });
        }

        if self.orientation == 2 || self.orientation == 3 {
            row_bits.reverse();
        }
        Tile::str_from_vec(&row_bits)
    }

    fn full_row(&self, r: usize) -> String {
        let mut row_bits: Vec<u8> = vec![];
        for i in 0..10 {
            row_bits.push(match self.orientation {
                0 => *self.tile.scans.get(r).unwrap().get(i).unwrap(),
                2 => *self.tile.scans.get(9 - r).unwrap().get(i).unwrap(),
                1 => *self.tile.scans.get(i).unwrap().get(9 - r).unwrap(),
                3 => *self.tile.scans.get(i).unwrap().get(r).unwrap(),
                _ => panic!("Unexpected orientation."),
            });
        }

        if self.orientation == 2 || self.orientation == 3 {
            row_bits.reverse();
        }
        Tile::str_from_vec(&row_bits)
    }
}

#[derive(Debug)]
struct Tile {
    index: u16,
    scans: Vec<Vec<u8>>
}

impl Tile {
    fn new(index: u16, scans: Vec<Vec<u8>>) -> Self {
        Tile { index, scans }
    }

    fn flipped(&self) -> Self {
        let mut scans = self.scans.to_owned();
        scans.reverse();
        Self{
            index:self.index, scans
        }
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

    fn str_from_vec(vals: &[u8]) -> String {
        let mut s = String::from("");
        for v in vals {
            s.push_str(match v {
                0 => ".",
                _ => "#",
            });
        }
        s
    }

    fn edge_str(&self, side: u8, shift: u8) -> String {
        let shifted = (side + shift) % 4;
        let mut key_vec = self.key_vec(shifted);
        if Self::edge_orientation(side) != Self::edge_orientation(shifted) {
            key_vec.reverse();
        }
        Tile::str_from_vec(&key_vec)
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
