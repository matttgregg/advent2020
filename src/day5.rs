use std::time::SystemTime;

use std::collections::HashSet;

pub fn run() {
    println!("Day5!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data5.txt");
    let contents = String::from_utf8_lossy(cbytes);

    parse_seat("BFFFBBFRRR");
    let (row, col, best) = find_max_seats(&contents);
    let my_seat = find_missing_seat(&contents);

    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();

    println!("Highest index at {}, {} => {}", row, col, best);
    println!("My seat at {}", my_seat);
    println!("Timed: {}us", timed);
}

fn find_missing_seat(seats: &str) -> i64 {
    let mut seated: HashSet<i64> = HashSet::new();
    let (mut min, mut max) = (0, 0);
    for seat in seats.lines() {
        let (_, _, i) = parse_seat(seat);
        if min == 0 || i < min {
            min = i;
        }

        if i > max {
            max = i
        }

        seated.insert(i);
    }

    for check in min..max {
        if !seated.contains(&check) && seated.contains(&(check - 1)) && seated.contains(&(check +1)) {
            return check;
        }
    }

    0
}

fn find_max_seats(seats: &str) -> (i64, i64, i64) {
    let (mut row, mut col, mut best) = (0, 0, 0);
    for seat in seats.lines() {
        let (r, c, i) = parse_seat(seat);
        if i > best {
            row = r;
            col = c;
            best = i;
        }
    }
    (row, col, best)
}

fn parse_seat(seat: &str) -> (i64, i64, i64) {
    if seat.trim().len() < 10 {
        (0, 0, 0)
    } else {
        let s = seat
            .trim()
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1");
        let row = i64::from_str_radix(&s[0..7], 2);
        let col = i64::from_str_radix(&s[7..10], 2);

        match (row.ok(), col.ok()) {
            (Some(r), Some(c)) => (r, c, 8 * r + c),
            _ => (0, 0, 0),
        }
    }
}
