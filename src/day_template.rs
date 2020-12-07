use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> String {
    let cbytes = include_bytes!("../data/data<DAY>.txt");
    String::from_utf8_lossy(cbytes).to_string()
}

pub fn run() {
    print_day();

    let start = SystemTime::now();

    // Let's do this...

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
