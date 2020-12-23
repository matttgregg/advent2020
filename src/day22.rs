use std::collections::{HashSet, VecDeque};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::time::SystemTime;
use std::hash::{Hash, Hasher};

use advent2020::{crab, fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data22.txt")
}

pub fn run() {
    print_day(22);

    let start = SystemTime::now();

    // Let's do this...

    let (winner, score) = winner_simple(data());
    let (recursive_winner, recursive_score) = run_rgame(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    println!(
        "The winner of the simple game is player {} with {}",
        winner, fmt_bright(&score)
    );
    println!(
        "The winner of the recursive game is player {} with {}",
        recursive_winner,
        fmt_bright(&recursive_score)
    );
    print_duration(timed);
}

fn winner_simple(game: &str) -> (String, u64) {
    let (winner, winning_deck) = play_game(game);
    let winner_name = if winner == 2 { crab() } else { String::from("ME") };
    (winner_name, score_deck(&winning_deck)) 
}

fn score_deck(deck: &VecDeque<u64>) -> u64 {
    let max = deck.len();
    let mut total_score = 0;
    for (i, card) in deck.iter().enumerate() {
        let multiplier = u64::try_from(max - i).unwrap();
        let score = multiplier * card;
        total_score += score;
    }

    total_score
}

fn play_game(game: &str) -> (u8, VecDeque<u64>) {
    let (mut deck_one, mut deck_two) = read_decks(game);

    while !deck_one.is_empty() && !deck_two.is_empty() {
        let (card1, card2) = (deck_one.pop_front().unwrap(), deck_two.pop_front().unwrap());
        if card1 > card2 {
            deck_one.push_back(card1);
            deck_one.push_back(card2);
        } else {
            deck_two.push_back(card2);
            deck_two.push_back(card1);
        }
    }

    if deck_one.is_empty() {
        (2, deck_two)
    } else {
        (1, deck_one)
    }
}

fn run_rgame(game: &str) -> (String, u64) {
    let (deck_one, deck_two) = read_decks(game);
    let (winner, score) = play_rgame(deck_one, deck_two);

    let winner_name = if winner == 2 { crab() } else { String::from("ME") };
    (winner_name, score)
}

fn deck_key(deck_one: &VecDeque<u64>, deck_two: &VecDeque<u64>) -> u64 {
    let mut s = DefaultHasher::new();
    (deck_one, deck_two).hash(&mut s);
    s.finish()
}

fn copy_deck(from: &VecDeque<u64>, count: u64) -> VecDeque<u64> {
    let card_count: usize = usize::try_from(count).unwrap();
    let mut new_deck = VecDeque::new();
    for card in 0..card_count {
        new_deck.push_back(*from.get(card).unwrap());
    }

    new_deck
}

fn play_rgame(mut deck_one: VecDeque<u64>, mut deck_two: VecDeque<u64>) -> (u8, u64) {
    let mut games_seen = HashSet::new();
    let mut winner = 0;
    while !deck_one.is_empty() && !deck_two.is_empty() {
        // Have we seen this combination before?
        let deck_key = deck_key(&deck_one, &deck_two);
        if games_seen.contains(&deck_key) {
            winner = 1;
            break;
        } else {
            games_seen.insert(deck_key);
            let (card1, card2) = (deck_one.pop_front().unwrap(), deck_two.pop_front().unwrap());
            if usize::try_from(card1).unwrap() <= deck_one.len()
                && usize::try_from(card2).unwrap() <= deck_two.len()
            {
                // We *can recurse!
                let sub_deck1 = copy_deck(&deck_one, card1);
                let sub_deck2 = copy_deck(&deck_two, card2);
                let (sub_winner, _) = play_rgame(sub_deck1, sub_deck2);
                winner = sub_winner;
            } else {
                // We can't recurse. Highest wins.
                winner = if card1 > card2 { 1 } else { 2 };
            }

            // Winner collects the cards.
            if winner == 1 {
                deck_one.push_back(card1);
                deck_one.push_back(card2);
            } else {
                deck_two.push_back(card2);
                deck_two.push_back(card1);
            }
        }
    }

    // If either card deck is empty, they are the winner, work out the score.
    // Otherwise, we must have finshed due to a repeating position.
    if deck_two.is_empty() {
        (1, score_deck(&deck_one))
    } else if deck_one.is_empty() {
        (2, score_deck(&deck_two))
    } else if winner == 1 {
        (1, score_deck(&deck_one))
    } else if winner == 2 {
        (2, score_deck(&deck_two))
    } else {
        panic!("Inavlid game result.");
    }
}

fn read_decks(game: &str) -> (VecDeque<u64>, VecDeque<u64>) {
    let mut first_player = true;
    let mut deck_one = VecDeque::new();
    let mut deck_two = VecDeque::new();
    for line in game.lines() {
        if let Ok(card) = line.parse::<u64>() {
            if first_player {
                deck_one.push_back(card);
            } else {
                deck_two.push_back(card);
            }
        }

        if line.starts_with("Player 2") {
            first_player = false;
        }
    }

    (deck_one, deck_two)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small() {
        let small_game = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!((crab(), 306), winner_simple(&small_game));
        assert_eq!((crab(), 291), run_rgame(&small_game));
    }

    #[test]
    fn test_all() {
        assert_eq!((crab(), 32783), winner_simple(data()));
        assert_eq!((crab(), 33455), run_rgame(data()));
    }
}
