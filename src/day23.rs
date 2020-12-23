use std::time::SystemTime;
use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    "614752839"
}

pub fn run() {
    print_day(23);

    let start = SystemTime::now();

    // Let's do this...
    let demo_data = "389125467";
    let demo_cups_10 = play_until(&demo_data, 10);
    let demo_cups_100 = play_until(&demo_data, 10);
    
    println!("Demo, after 10 = {}", result_for(&demo_cups_10));
    println!("Demo, after 100 = {}", result_for(&demo_cups_100));
    
    //let demo_big = play_big_game(&demo_data, 10_000_000);
    let demo_big = play_big_game(&demo_data, 10, 10);
    println!("Finished big demo game.");

    let cups_100 = play_until(data(), 100);
    println!("Game after 100 moved is {}", fmt_bright(&result_for(&cups_100)));

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn play_big_game(init: &str, cups: usize, rounds: usize)  {
    // Now try the 10 million game.
    let (mut game, mut focus) = init_smart_game(init, 10);
    println!("Game: {:?}/{}", game, focus);
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
    for i in init.len()..=max {
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

fn result_for(game: &VecDeque<i32>) -> String {
    let mut after_one = false;
    let mut second_part = String::from("");
    let mut first_part = String::from("");

    for val in game {
        if *val == 1 {
            after_one = true;
        } else if after_one {
            first_part.push_str(format!("{}", val).as_str());
        } else {
            second_part.push_str(format!("{}", val).as_str());
        }
    }
    format!("{}{}", first_part, second_part)
}

fn play_until(init: &str, rounds: usize) -> VecDeque<i32> {
    let mut game = init_game(init);
    for _ in 0..rounds {
        game = play_round(game); 
    }
    game
}

fn init_game(init: &str) -> VecDeque<i32> {
    let mut game = VecDeque::new();
    for c in init.chars() {
        if let Ok(val) = c.to_string().parse::<i32>() {
            game.push_back(val);
        }
    }
    game
}


fn play_round(mut game: VecDeque<i32>) -> VecDeque<i32> {
    // Pop chars
    let mut next_game = VecDeque::new();
    let curr = game.pop_front().unwrap();
    // Pop three to the side
    let mut side = VecDeque::new();
    let mut popped = HashSet::new();
    for _ in 0..3 {
        let pop = game.pop_front().unwrap();
        popped.insert(pop);
        side.push_back(pop);
    }

    // What's the next item?
    let mut pop_target =  (curr - 2) % 9 + 1;
    if pop_target < 1 {
        pop_target += 9;
    }
    while popped.contains(&pop_target) {
        pop_target = (pop_target - 2) % 9 + 1;
        if pop_target < 1 {
            pop_target += 9;
        }
    }

    for v in game {
        next_game.push_back(v);
        if v == pop_target {
            // Push the saved cards back in here.
            for saved in &side {
                next_game.push_back(*saved);
            }
        }
    }

    // Put the current value back at the end. (i.e. rotate clockwise)
    next_game.push_back(curr);

    next_game
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
