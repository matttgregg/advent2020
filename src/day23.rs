use std::time::SystemTime;
use std::collections::{HashSet};
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
    let next_after_one = *game.get(after_one).unwrap();

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

fn from_one(count: usize, cups: &[usize]) -> String {
    let mut code = String::from("");

    let mut at_cup = 1;
    // Note we start at one, because we skip the '1'
    for _ in 1..count {
        let move_to = cups.get(at_cup).unwrap();
        code.push_str(format!("{}", *move_to).as_str());
        at_cup = *move_to;
    }
    code

}

fn smart_round(max: usize, cups: &mut Vec<usize>, focus: usize) -> usize {
    // Pop three after the focus.
    let mut popped = HashSet::new();
    
    let first_popped = *cups.get(focus).unwrap();
    popped.insert(first_popped);

    // Two steps to find the last popped.
    let mut last_popped = *cups.get(first_popped).unwrap();
    popped.insert(last_popped);
    last_popped = *cups.get(last_popped).unwrap(); 
    popped.insert(last_popped);

    // The focus is reconnected to the following entry.
    cups[focus] = cups[last_popped];

    let mut insert_at = focus - 1;
    if insert_at == 0 {
        insert_at = max;
    }
    
    while popped.contains(&insert_at) {
        insert_at -= 1;
        if insert_at == 0 {
            insert_at = max;
        }
    }

    // Stitch in the popped values.
    let end_of_stitch = *cups.get(insert_at).unwrap();
    cups[insert_at] = first_popped;
    cups[last_popped] = end_of_stitch;

    // Return the new focus.
    cups[focus]
}

fn init_smart_game(init: &str, max: usize) -> (Vec<usize>, usize) {
    let mut game = vec![0; max + 1];
    let vals =  init.chars().filter_map(|x| x.to_string().parse::<usize>().ok());
    let mut previous = None;
    let mut first = 0;
    for val in vals {
        if let Some(p) = previous  {
            game[p] = val;
        } else {
            first = val;
        }
        previous = Some(val)
    }

    // Now, need to add the remainder.
    for i in (init.len() + 1)..=max {
        if let Some(p) = previous  {
            game[p] = i;
        }
        previous = Some(i)
    }

    // Finally, the last value loops.
    if let Some(p) = previous {
        game[p] = first;
    }
    (game, first)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    /*
    #[test]
    fn test_small() {
        
        let demo_data = "389125467";
        let demo_cups_10 = play_until(&demo_data, 10);
        let demo_cups_100 = play_until(&demo_data, 10);
        
        println!("Demo, after 10 = {}", result_for(&demo_cups_10));
        println!("Demo, after 100 = {}", result_for(&demo_cups_100));
        
        //let demo_big = play_big_game(&demo_data, 10_000_000);
        let demo_big = play_big_game(&demo_data, 1_000_000, 10_000_000);
        println!("Finished big demo game.");
    }
    */

    #[test]
    fn test_all() {}
}
