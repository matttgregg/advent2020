use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> (u64, u64) {
    (8_252_394, 6_269_621)
}

pub fn run() {
    print_day(25);

    let start = SystemTime::now();

    // Let's do this...

    // Example
    crack(5764801, 17807724);

    // Full
    crack(data().0, data().1);

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn crack(card: u64, door: u64) {
    // The card/door keys are found by transforming 7.
    let card_lp = crack_transform(7, card);
    assert_eq!(card, transform(7, card_lp));

    let door_lp = crack_transform(7, door);
    assert_eq!(door, transform(7, door_lp));


    println!("Found card/door loops: {} and {}", card_lp, door_lp);
    // The encryption key is found by either transforming the others key.
    println!("Card generates encryption key: {}", transform(door, card_lp));
    println!("Door generates encryption key: {}", transform(card, door_lp));

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
    fn test_all() {}
}
