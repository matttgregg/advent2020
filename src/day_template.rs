use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

pub fn run() {
    print_day();

    let start = SystemTime::now();
    /*
    let cbytes = include_bytes!("../data/data<DAY>.txt");
    let contents = String::from_utf8_lossy(cbytes);
    */

    let timed = SystemTime::now().duration_since(start).unwrap();

    print_duration(timed);
}

mod tests {
    use super::*;

    #[test]
    fn test_all() {}
}
