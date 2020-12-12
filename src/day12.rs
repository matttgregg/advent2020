use std::mem;
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data12.txt")
}

pub fn run() {
    print_day(12);

    let start = SystemTime::now();

    // Let's do this...
    let mut ferry1 = Boat::new();
    let mut ferry2 = BoatWithWaypoint::new();
    voyage(data(), &mut ferry1);
    voyage(data(), &mut ferry2);

    let data_small = "F10
N3
F7
R90
F11";
    let mut small_ferry1 = Boat::new();
    let mut small_ferry2 = BoatWithWaypoint::new();
    voyage(&data_small, &mut small_ferry1);
    voyage(&data_small, &mut small_ferry2);

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn voyage(route: &str, ferry: &mut impl Navigable) {
    for mv in route.lines() {
        let digits = mv.chars().filter(|x| x.is_digit(10) || (*x == '-')).collect::<String>();
        let d = digits.parse::<i64>();

        if let Ok(dist) = d {
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
        }
    }
    println!("{}", ferry.announce()); 
}

trait Navigable {
    fn forward(&mut self, d: i64);
    fn north(&mut self, d: i64);
    fn south(&mut self, d: i64);
    fn east(&mut self, d: i64);
    fn west(&mut self, d: i64);
    fn left(&mut self, d: i64);
    fn right(&mut self, d: i64);
    fn announce(& self) -> String;
}

struct BoatWithWaypoint {
    x: i64,
    y: i64,
    waypoint_x: i64,
    waypoint_y: i64,
}

impl BoatWithWaypoint {
    fn new() -> Self {
        BoatWithWaypoint { x:0,y:0, waypoint_x:10, waypoint_y:1}
    }
}

impl Navigable for BoatWithWaypoint {
    fn forward(&mut self, d: i64) {
        self.x += d * self.waypoint_x;
        self.y += d * self.waypoint_y;
    }
    
    fn left(&mut self, angle: i64) {
        match angle {
            0 => {},
            90 => { mem::swap(&mut self.waypoint_x, &mut self.waypoint_y); self.waypoint_x *= -1; }
            180 => { self.waypoint_x *= -1; self.waypoint_y *= -1;}
            270 => { mem::swap(&mut self.waypoint_x, &mut self.waypoint_y); self.waypoint_y *= -1; }
            _ => { println!("Unexpected angle {}", angle); panic!("Invalid angle."); }
        }
    }
    
    fn right(&mut self, angle: i64) {
        match angle {
            0 => {},
            90 => { mem::swap(&mut self.waypoint_x, &mut self.waypoint_y); self.waypoint_y *= -1; }
            180 => { self.waypoint_x *= -1; self.waypoint_y *= -1;}
            270 => { mem::swap(&mut self.waypoint_x, &mut self.waypoint_y); self.waypoint_x *= -1; }
            _ => { println!("Unexpected angle {}", angle); panic!("Invalid angle."); }
        }
    }

    fn north(&mut self, d: i64) { self.waypoint_y += d; }
    fn south(&mut self, d: i64) { self.waypoint_y -= d; }
    fn east(&mut self, d: i64) { self.waypoint_x += d; }
    fn west(&mut self, d: i64) { self.waypoint_x -= d; }

    fn announce(& self) -> String { format!(" -> {},{} (Waypoint {}, {}) [{}]",
                                            self.x, self.y,
                                            self.waypoint_x, self.waypoint_y,
                                            self.x.abs() + self.y.abs()) }
    
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
}

impl Navigable for Boat {
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

    fn announce(& self) -> String { format!(" -> {},{} [{}]", self.x, self.y, self.x.abs() + self.y.abs()) }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
