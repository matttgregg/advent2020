use std::time::SystemTime;
use std::convert::TryFrom;

use advent2020::{fmt_bright, print_day, print_duration, crab};

fn data() -> &'static str {
    "614752839"
}

pub fn run() {
    print_day(23);

    let start = SystemTime::now();

    // Let's do this...
    let first_game = play_game(data(), 100);
    println!("Finished a suspiciously easy game with {}", fmt_bright(&first_game));
    let big_game = play_big_game(data(), 1_000_000, 10_000_000);

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Finally beaten that {} by finding my stars at {}", crab(), fmt_bright(&big_game));
    print_duration(timed);
}

fn play_big_game(init: &str, cups: usize, rounds: usize) -> u64 {
    let (mut game, mut focus) = init_smart_game(init, cups);
    for _ in 0..rounds {
        focus = smart_round(cups, &mut game, focus);
    }

    // Find left of one.
    let after_one = *game.get(1).unwrap();
    let next_after_one = *game.get(after_one as usize).unwrap();

    let grand_product = u64::try_from(after_one).unwrap() * u64::try_from(next_after_one).unwrap();
    println!("The values after 1 are: {} x {} ==> {}", after_one, next_after_one, grand_product);
    grand_product
}
fn play_game(init: &str, rounds: usize) -> String {
    let cups = init.len();
    let (mut game, mut focus) = init_smart_game(init, cups);
    for _ in 0..rounds {
        focus = smart_round(cups, &mut game, focus);
    }
    from_one(cups, &game)
} 

fn from_one(count: usize, cups: &[u32]) -> String {
    let mut code = String::from("");

    let mut at_cup = 1;
    // Note we start at one, because we skip the '1'
    for _ in 1..count {
        let move_to = cups.get(at_cup).unwrap();
        code.push_str(format!("{}", *move_to).as_str());
        at_cup = *move_to as usize;
    }
    code

}

fn smart_round(max: usize, cups: &mut [u32], focus: usize) -> usize {
    // Pop three after the focus.
    let first_popped = *cups.get(focus).unwrap() as usize;
    let middle_popped = *cups.get(first_popped).unwrap() as usize;
    let last_popped = *cups.get(middle_popped).unwrap() as usize;

    // The focus is reconnected to the following entry.
    cups[focus] = cups[last_popped as usize];

    let mut insert_at = focus - 1;
    if insert_at == 0 {
        insert_at = max;
    }
    
    while
        (insert_at == first_popped)
        || (insert_at == middle_popped)
        || (insert_at == last_popped) {
            insert_at -= 1;
            if insert_at == 0 {
                insert_at = max;
            }
        }

    // Stitch in the popped values.
    let end_of_stitch = *cups.get(insert_at).unwrap();
    cups[insert_at] = u32::try_from(first_popped).unwrap();
    cups[last_popped] = end_of_stitch;

    // Return the new focus.
    cups[focus] as usize
}

fn init_smart_game(init: &str, max: usize) -> (Box<[u32]> , usize) {
    let mut game = vec![0_u32; 1_000_001].into_boxed_slice();
    let vals =  init.chars().filter_map(|x| x.to_string().parse::<u32>().ok());
    let mut previous = None;
    let mut first = 0;
    for val in vals {
        if let Some(p) = previous  {
            game[p as usize] = val;
        } else {
            first = val;
        }
        previous = Some(val)
    }

    // Now, need to add the remainder.
    for i in (init.len() + 1)..=max {
        if let Some(p) = previous  {
            game[p as usize] = u32::try_from(i).unwrap();
        }
        previous = Some(u32::try_from(i).unwrap())
    }

    // Finally, the last value loops.
    if let Some(p) = previous {
        game[p as usize] = first;
    }
    (game, first as usize)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small_game() {
        
        let demo_data = "389125467";
        assert_eq!("92658374", play_game(&demo_data, 10));
        assert_eq!("67384529", play_game(&demo_data, 100));
    }

    #[test]
    fn test_big_game() {
        let demo_data = "389125467";
        assert_eq!(149245887792, play_big_game(&demo_data, 1_000_000, 10_000_000));
    }

    #[test]
    fn test_all() {
        assert_eq!("89372645", play_game(data(), 100));
        assert_eq!(21273394210, play_big_game(data(), 1_000_000, 10_000_000));
    }
}
