use pest::Parser;
use pest_derive::Parser;

use std::time::SystemTime;
use std::collections::{HashSet, HashMap};

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data21.txt")
}

pub fn run() {
    print_day(21);

    let start = SystemTime::now();

    // Let's do this...
    let data_small = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    parse_foods(&data_small);
    parse_foods(data());

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn parse_foods(data: &str) {
    let parsed = DParser::parse(Rule::file, data)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut foods: Vec<FoodStuff> = vec![];
    for line in parsed.into_inner() {
        let mut food = FoodStuff::new();
        for part in line.into_inner() {
            match part.as_rule() {
                Rule::ingredient => { food.ingredients.insert(part.as_str().to_string()); },
                Rule::allergen => { food.allergens.insert(part.as_str().to_string()); },
                _ => panic!("Unexpected rule."),
            }
        }
        foods.push(food);
    }

    // Find all the allergens
    let mut all_allergens: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, food) in foods.iter().enumerate() {
        for allergen in &food.allergens {
            all_allergens.entry(allergen.to_string()).or_insert(vec![]).push(i)
        }
    }

    println!("Found allergens: {:?}", all_allergens);
    let mut dangerous_foods = HashSet::new();

    // For each allergen, we find the intersection of ingredients that can contain it.
    for (allergen, indexed_foods) in all_allergens {
        let mut foods_intersect = HashSet::new();
        for food in indexed_foods {
            let ingredients = foods.get(food).unwrap().ingredients.iter().collect();
            if foods_intersect.is_empty() {
                foods_intersect = ingredients;
            } else {
                foods_intersect = foods_intersect.intersection(&ingredients).copied().collect();
            }
        }
        println!("Allergen {} must be in {:?}", allergen, foods_intersect);
        for food in foods_intersect {
            dangerous_foods.insert(food);
        }
    }
    
    println!("Found dangerous foods: {:?}", dangerous_foods);

    let mut safe_foods = HashSet::new();
    let mut safe_uses = 0;
    for food in &foods {
        for ingredient in &food.ingredients {
            if !dangerous_foods.contains(&ingredient) {
                safe_foods.insert(ingredient);
                safe_uses += 1;
            }
        }
    } 

    println!("Found safe foods: {:?}", safe_foods);
    println!("Total safe ingredients: {}", safe_uses);
}

#[derive(Debug)]
struct FoodStuff {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FoodStuff {
    fn new() -> Self {
        FoodStuff{ ingredients:HashSet::new(), allergens: HashSet::new() }
    }
}

#[derive(Parser)]
#[grammar = "parsers/day21.pest"]
pub struct DParser {}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all() {}
}
