use std::time::SystemTime;
use std::iter;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data17.txt")
}

pub fn run() {
    print_day(17);

    let data_small = ".#.
..#
###";

    let start = SystemTime::now();
    run_cube(&data_small, 6);
    run_cube(data(), 6);

    // Let's do this...

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn run_cube(init: &str, iters: usize) {
    /*
    let cube_array = CubeArray::new((-1, -1, -1), (1,1,1));

    println!("Built cube : {:?}", cube_array.config);

    for (i, _c) in cube_array.cubes.iter().enumerate() {
        let unflattened = cube_array.config.unflatten(i);
        let reflattened = cube_array.config.try_flatten(unflattened.0, unflattened.1, unflattened.2);
        println!("[{}] == {:?} <==> {:?}", i, unflattened, reflattened);
    }

    // Look at neighbours:
    let pt = (-1, -1, -1);
    println!("{:?} has neighbours:", &pt);
    for n in cube_array.config.neigbours(cube_array.config.flatten(pt.0, pt.1, pt.2)) {
        println!("\t{:?}", cube_array.config.unflatten(n));
    }
    */

    // How big is the cube data?
    let sizex = (init.lines().count() + 2*iters) as i32;
    let offset = -(iters as i32);

    let mut cube_array = CubeArray::new((offset, offset, offset), (sizex, sizex, (2 * iters) as i32));

    // Initialize the array.
    for (x, line) in init.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            let ix = cube_array.config.flatten(x as i32, y as i32, 0);
            cube_array.cubes[ix] = match ch {
                '#' => 1,
                _ => 0,
            };
        }
    }

    // Print & count the cube:
    let mut active = 0;
    for (ix, c) in cube_array.cubes.iter().enumerate() {
        if *c == 1 {
            active += 1;
        }
    }
    println!("Found {} active cubes.", active);

    // Now iterate:
    for iteration in 0..iters {
        let mut set_zero = vec![];
        let mut set_one = vec![];
        println!("[{}] {} active.", iteration, active);

        // Go through each cube
        for (ix, c) in cube_array.cubes.iter().enumerate() {
            let mut active_neighbours = 0;
            for n in cube_array.config.neigbours(ix) {
                active_neighbours += cube_array.cubes[n];
            }
            if *c == 1 && (active_neighbours < 2 || active_neighbours > 3) {
                set_zero.push(ix);
            } else if *c == 0 && active_neighbours == 3 {
                set_one.push(ix);
            }
        }

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

    println!("Final active: {}", active);



} 

#[derive(Debug)]
struct CubeConfig {
    wx: usize,
    wy: usize,
    wz: usize,
    offsetx: i32,
    offsety: i32,
    offsetz: i32
}

impl CubeConfig {
    fn flatten(&self, x: i32, y: i32, z: i32) -> usize {
        let zz: usize = (z - self.offsetz) as usize;
        let yy: usize = ((y - self.offsety) as usize) * self.wz;
        let xx: usize = ((x - self.offsetx) as usize) * self.wz * self.wy;
        xx + yy + zz
    }
    
    fn try_flatten(&self, x: i32, y: i32, z: i32) -> Option<usize> {
        if z < self.offsetz || y < self.offsety || x < self.offsetx {
            return None;
        }

        let zz: usize = (z - self.offsetz) as usize;
        let yy: usize = (y - self.offsety) as usize;
        let xx: usize = (x - self.offsetx) as usize;
        if zz >= self.wz || yy >= self.wy || xx >= self.wx {
            None
        } else {
            Some(xx * self.wy * self.wz + yy * self.wz + zz)
        }
    }

    fn neigbours(&self, i: usize) -> Vec<usize> {
        let points = vec![
            (-1, -1, -1),
            (-1, 0, -1),
            (-1, 1, -1),
            (-1, -1, 0),
            (-1, 0, 0),
            (-1, 1, 0),
            (-1, -1, 1),
            (-1, 0, 1),
            (-1, 1, 1),
            (0, -1, -1),
            (0, 0, -1),
            (0, 1, -1),
            (0, -1, 0),
            // (0, 0, 0), - Note here we skip the point itself.
            (0, 1, 0),
            (0, -1, 1),
            (0, 0, 1),
            (0, 1, 1),
            (1, -1, -1),
            (1, 0, -1),
            (1, 1, -1),
            (1, -1, 0),
            (1, 0, 0),
            (1, 1, 0),
            (1, -1, 1),
            (1, 0, 1),
            (1, 1, 1),
        ];
        let (x, y, z) = self.unflatten(i);
        points.into_iter().map(|(p,q,r)| (x + p, y + q, z + r)).filter_map(|(p,q,r)| self.try_flatten(p, q, r)).collect()
   }

    fn unflatten(&self, i: usize) -> (i32, i32, i32) {
        let x: i32 = (i / (self.wy * self.wz)) as i32;
        let y: i32 = ((i / (self.wz)) % self.wy) as i32;
        let z: i32 = (i % self.wz) as i32;
        (x + self.offsetx, y + self.offsety, z + self.offsetz)
    }
}


struct CubeArray {
    config: CubeConfig,
    cubes: Vec<u8>,
}

impl CubeArray {
    fn new(min: (i32, i32, i32), max: (i32, i32, i32)) -> Self {
        let config = CubeConfig {
            wx: 1 + (max.0 - min.0).abs() as usize,
            wy: 1 + (max.1 - min.1).abs() as usize,
            wz: 1 + (max.2 - min.2).abs() as usize,
            offsetx: min.0,
            offsety: min.1,
            offsetz: min.2
        };
        let cubes = vec![0_u8; config.flatten(max.0, max.1, max.2) + 1];
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
    fn test_all() {}
}
