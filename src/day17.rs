use std::time::SystemTime;
use std::convert::TryInto;
use std::collections::HashSet;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data17.txt")
}

pub fn run() {
    print_day(17);

    let start = SystemTime::now();
    let res1 = run_cube(data(), 6, false);
    println!("Conway Cube energy output => {}", fmt_bright(&res1));
    let res2 = run_cube(data(), 6, true);
    println!("Conway HyperCube energy output => {}", fmt_bright(&res2));

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn run_cube(init: &str, iters: usize, with_w: bool) -> u32 {
    // How big is the cube data?
    let sizex = (init.lines().count() + 2*iters).try_into().unwrap();
    let offset: Result<i32, _> = iters.try_into();
    let wz = iters.try_into().unwrap();
    let ww = if with_w { wz } else { 0_i32 };

    let mut cube_array = CubeArray::new((-ww, -offset.unwrap(), -offset.unwrap(), -wz), (ww, sizex, sizex, wz));

    // Initialize the array.
    let mut live_cubes = vec![];
    let mut seen: HashSet<usize> = HashSet::new();
    for (x, line) in init.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            let ix = cube_array.config.flatten(0, x.try_into().unwrap(), y.try_into().unwrap(), 0);
            cube_array.cubes[ix] = match ch {
                '#' => {
                    if !seen.contains(&ix) {
                        seen.insert(ix);
                        live_cubes.push(ix);
                    }
                    for ixn in cube_array.config.neigbours(ix) {
                        if !seen.contains(&ixn) {
                            seen.insert(ixn);
                            live_cubes.push(ixn);
                        }
                    }
                    1   
                }
                _ => 0,
            };
        }
    }

    // Print & count the cube:
    let mut active = 0;
    for ix in &live_cubes {
        if cube_array.cubes[*ix] == 1 {
            active += 1;
        }
    }

    // Now iterate:
    for iteration in 0..iters {
        let mut set_zero = vec![];
        let mut set_one = vec![];
        println!("[{}] {} active.", iteration, active);
        let mut next_live_cubes = vec![];
        let mut seen: HashSet<usize> = HashSet::new();

        // Go through each cube
        for ix in &live_cubes {
            let c = cube_array.cubes[*ix];
            let mut active_neighbours = 0;
            for n in cube_array.config.neigbours(*ix) {
                active_neighbours += cube_array.cubes[n];
            }
            if c == 1 {
                if (2..=3).contains(&active_neighbours) {
                    if !seen.contains(ix) {
                        seen.insert(*ix);
                        next_live_cubes.push(*ix);
                    }
                    for inx in &cube_array.config.neigbours(*ix) {
                        if !seen.contains(inx) {
                            seen.insert(*inx);
                            next_live_cubes.push(*inx);
                        }
                    }
                } else {
                    set_zero.push(*ix);
                }
            } else if c == 0 && active_neighbours == 3 {
                set_one.push(*ix);
                if !seen.contains(ix) {
                    seen.insert(*ix);
                    next_live_cubes.push(*ix);
                }
                for inx in &cube_array.config.neigbours(*ix) {
                    if !seen.contains(inx) {
                        seen.insert(*inx);
                        next_live_cubes.push(*inx);
                    }
                }
            }
        }
        
        live_cubes = next_live_cubes;
        seen.clear();

        // Now update.
        for ix in set_zero {
            cube_array.cubes[ix] = 0;
            active -= 1;
        }

        for ix in set_one {
            cube_array.cubes[ix] = 1;
            active += 1;
        }

    }

    println!("[{}] {} active.", iters, active);
    active
} 

#[derive(Debug)]
struct CubeConfig {
    wx: usize,
    wy: usize,
    wz: usize,
    ww: usize,
    offsetx: i32,
    offsety: i32,
    offsetz: i32,
    offsetw: i32
}

impl CubeConfig {
    fn flatten(&self, w: i32, x: i32, y: i32, z: i32) -> usize {
        let zz: usize = (z - self.offsetz).try_into().unwrap();
        let yy: usize = (y - self.offsety).try_into().unwrap();
        let yy = yy * self.wz;
        let xx: usize = (x - self.offsetx).try_into().unwrap();
        let xx = xx * self.wz * self.wy;
        let ww: usize = (w - self.offsetw).try_into().unwrap();
        let ww = ww * self.wx * self.wy * self.wz;
        ww + xx + yy + zz
    }
    
    fn try_flatten(&self, w: i32, x: i32, y: i32, z: i32) -> Option<usize> {
        if z < self.offsetz || y < self.offsety || x < self.offsetx || w < self.offsetw {
            return None;
        }

        let zz: usize = (z - self.offsetz).try_into().unwrap();
        let yy: usize = (y - self.offsety).try_into().unwrap();
        let xx: usize = (x - self.offsetx).try_into().unwrap();
        let ww: usize = (w - self.offsetw).try_into().unwrap();
        if zz >= self.wz || yy >= self.wy || xx >= self.wx || ww >= self.ww {
            None
        } else {
            Some(ww * self.wx * self.wy * self.wz + xx * self.wy * self.wz + yy * self.wz + zz)
        }
    }

    fn neigbours(&self, i: usize) -> Vec<usize> {
        let points = vec![
            (-1, -1, -1, -1),
            (-1, -1, 0, -1),
            (-1, -1, 1, -1),
            (-1, -1, -1, 0),
            (-1, -1, 0, 0),
            (-1, -1, 1, 0),
            (-1, -1, -1, 1),
            (-1, -1, 0, 1),
            (-1, -1, 1, 1),
            (-1, 0, -1, -1),
            (-1, 0, 0, -1),
            (-1, 0, 1, -1),
            (-1, 0, -1, 0),
            (-1, 0, 0, 0), 
            (-1, 0, 1, 0),
            (-1, 0, -1, 1),
            (-1, 0, 0, 1),
            (-1, 0, 1, 1),
            (-1, 1, -1, -1),
            (-1, 1, 0, -1),
            (-1, 1, 1, -1),
            (-1, 1, -1, 0),
            (-1, 1, 0, 0),
            (-1, 1, 1, 0),
            (-1, 1, -1, 1),
            (-1, 1, 0, 1),
            (-1, 1, 1, 1),
            (0, -1, -1, -1),
            (0, -1, 0, -1),
            (0, -1, 1, -1),
            (0, -1, -1, 0),
            (0, -1, 0, 0),
            (0, -1, 1, 0),
            (0, -1, -1, 1),
            (0, -1, 0, 1),
            (0, -1, 1, 1),
            (0, 0, -1, -1),
            (0, 0, 0, -1),
            (0, 0, 1, -1),
            (0, 0, -1, 0),
            // (0, 0, 0, 0), -- Exclude the point itself
            (0, 0, 1, 0),
            (0, 0, -1, 1),
            (0, 0, 0, 1),
            (0, 0, 1, 1),
            (0, 1, -1, -1),
            (0, 1, 0, -1),
            (0, 1, 1, -1),
            (0, 1, -1, 0),
            (0, 1, 0, 0),
            (0, 1, 1, 0),
            (0, 1, -1, 1),
            (0, 1, 0, 1),
            (0, 1, 1, 1),
            (1, -1, -1, -1),
            (1, -1, 0, -1),
            (1, -1, 1, -1),
            (1, -1, -1, 0),
            (1, -1, 0, 0),
            (1, -1, 1, 0),
            (1, -1, -1, 1),
            (1, -1, 0, 1),
            (1, -1, 1, 1),
            (1, 0, -1, -1),
            (1, 0, 0, -1),
            (1, 0, 1, -1),
            (1, 0, -1, 0),
            (1, 0, 0, 0), 
            (1, 0, 1, 0),
            (1, 0, -1, 1),
            (1, 0, 0, 1),
            (1, 0, 1, 1),
            (1, 1, -1, -1),
            (1, 1, 0, -1),
            (1, 1, 1, -1),
            (1, 1, -1, 0),
            (1, 1, 0, 0),
            (1, 1, 1, 0),
            (1, 1, -1, 1),
            (1, 1, 0, 1),
            (1, 1, 1, 1),
        ];
        let (shift_w, shift_x, shift_y, shift_z) = self.unflatten(i);
        points.into_iter()
            .map(|(w,x,y,z)| (w + shift_w, x + shift_x, y + shift_y, z + shift_z))
            .filter_map(|(w, x, y, z)| self.try_flatten(w, x, y, z))
            .collect()
   }

    fn unflatten(&self, i: usize) -> (i32, i32, i32, i32) {
        let shifted_w: i32 = (i / (self.wx * self.wy * self.wz)).try_into().unwrap();
        let shifted_x: i32 = ((i / (self.wy * self.wz)) % self.wx).try_into().unwrap();
        let shifted_y: i32 = ((i / (self.wz)) % self.wy).try_into().unwrap();
        let shifted_z: i32 = (i % self.wz).try_into().unwrap();
        (shifted_w + self.offsetw, shifted_x + self.offsetx, shifted_y + self.offsety, shifted_z + self.offsetz)
    }
}


struct CubeArray {
    config: CubeConfig,
    cubes: Vec<u8>,
}

impl CubeArray {
    fn new(min: (i32, i32, i32, i32), max: (i32, i32, i32, i32)) -> Self {
        let config = CubeConfig {
            ww: 1 + (max.0 - min.0).abs() as usize,
            wx: 1 + (max.1 - min.1).abs() as usize,
            wy: 1 + (max.2 - min.2).abs() as usize,
            wz: 1 + (max.3 - min.3).abs() as usize,
            offsetw: min.0,
            offsetx: min.1,
            offsety: min.2,
            offsetz: min.3
        };
        let cubes = vec![0_u8; config.flatten(max.0, max.1, max.2, max.3) + 1];
        CubeArray {
            config,
            cubes
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small() {
        let data_small = ".#.
..#
###";

        assert_eq!(112, run_cube(&data_small, 6, false));
        assert_eq!(848, run_cube(&data_small, 6, true));
    }

    #[test]
    fn test_all() {
        assert_eq!(213, run_cube(data(), 6, false));
        assert_eq!(1624, run_cube(data(), 6, true));
    }
}
