use std::error::Error;
use std::iter::Iterator;
use std::time::SystemTime;

pub fn run() {
    println!("Day3!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data3.txt");
    let contents = String::from_utf8_lossy(cbytes);

    let trajs: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut runs: Vec<usize> = Vec::new();

    for (x, y) in trajs {
        runs.push(run_xy_string(&contents, x, y).unwrap_or(0));
    }
    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();
    println!("Grand product {}", runs.iter().product::<usize>());
    println!("Timed: {}us", timed);
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

    println!("Going {}, {}, hit {} trees :(", x, y, trees);

    Ok(trees)
}
