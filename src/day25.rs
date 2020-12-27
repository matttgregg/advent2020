use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> (u64, u64) {
    (8_252_394, 6_269_621)
}

pub fn run() {
    print_day(25);

    let start = SystemTime::now();

    // Let's do this...
    let key = crack(data().0, data().1);
    println!("The door/card encryption key is {}", fmt_bright(&key));

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn crack(card: u64, door: u64) -> u64 {
    // The card/door keys are found by transforming 7.
    let card_lp = crack_transform(7, card);
    assert_eq!(card, transform(7, card_lp));

    let door_lp = crack_transform(7, door);
    assert_eq!(door, transform(7, door_lp));

    // The encryption key is found by either transforming the others key.
    transform(door, card_lp)
}

fn crack_transform(subject: u64, target: u64) -> usize {
    let mut value = subject;
    let mut lp = 1;
    loop {
        if value == target {
            break lp
        }
        lp += 1;
        value = (value * subject) % 20_201_227_u64;
    }
}

fn transform(subject: u64, lp: usize) -> u64 {
    let mut value: u64 = 1;
    for _ in 0..lp {
        value = (value * subject) % 20_201_227_u64;
    }
    value
} 

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(14_897_079, crack(5_764_801, 17_807_724));
    }

    #[test]
    fn test_all() {
        assert_eq!(181800, crack(data().0, data().1));
    }
}
