use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data12.txt")
}

pub fn run() {
    print_day(12);

    let start = SystemTime::now();

    // Let's do this...
    voyage(data());

    /*
    let data_small = "F10
N3
F7
R90
F11";
    voyage(&data_small);
    */

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

pub fn voyage(route: &str) {
    let mut ferry = Boat::new();
    for mv in route.lines() {
        println!("{}", mv);
        let digits = mv.chars().filter(|x| x.is_digit(10) || (*x == '-')).collect::<String>();
        let d = digits.parse::<i64>();

        if let Ok(dist) = d {
            println!("Pointing:{}", ferry.dir);
            match &mv[0..1] {
                "N" => ferry.north(dist),
                "S" => ferry.south(dist),
                "E" => ferry.east(dist),
                "W" => ferry.west(dist),
                "L" => ferry.left(dist),
                "R" => ferry.right(dist),
                "F" => ferry.forward(dist),
                _ => {},
            };
            println!(" -> {},{} [{}]", ferry.x, ferry.y, ferry.x.abs() + ferry.y.abs());
        }
    }
}

struct Boat {
    x: i64,
    y: i64,
    dir: i64
}

impl Boat {
    fn new() -> Self {
        Boat { x:0,y:0, dir:90}
    }

    fn forward(&mut self, d: i64) {
        match self.dir {
            0 => { self.north(d);},
            90 => { self.east(d);},
            180 => { self.south(d);},
            270 => { self.west(d);},
            _ => {
                println!("Direction! {}", self.dir);
                panic!("Unexpected direction.");
            },
        }
    }
    
    fn left(&mut self, angle: i64) {
        self.dir -= angle;
        self.dir %= 360;
        if self.dir < 0 {
            self.dir += 360;
        }
    }
    
    fn right(&mut self, angle: i64) {
        self.dir += angle;
        self.dir %= 360;
        if self.dir < 0 {
            self.dir += 360;
        }
    }

    fn north(&mut self, d: i64) { self.y += d; }
    fn south(&mut self, d: i64) { self.y -= d; }
    fn east(&mut self, d: i64) { self.x += d; }
    fn west(&mut self, d: i64) { self.x -= d; }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
