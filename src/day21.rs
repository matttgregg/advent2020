use pest::Parser;
use pest_derive::Parser;

use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

use advent2020::{fmt_bright, print_day, print_duration};

fn data() -> &'static str {
    include_str!("../data/data21.txt")
}

pub fn run() {
    print_day(21);

    let start = SystemTime::now();

    // Let's do this...
    let (safe_uses, canonical_list) = parse_foods(data());
    println!("Safe ingredients are used {} times.", fmt_bright(&safe_uses));
    println!("Canonical Dangerous Ingredient List (CDIL): {}", fmt_bright(&canonical_list));

    let timed = SystemTime::now().duration_since(start).unwrap();
    print_duration(timed);
}

fn parse_foods(data: &str) -> (u64, String) {
    let parsed = DParser::parse(Rule::file, data)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut foods: Vec<FoodStuff> = vec![];
    for line in parsed.into_inner() {
        let mut food = FoodStuff::new();
        for part in line.into_inner() {
            match part.as_rule() {
                Rule::ingredient => {
                    food.ingredients.insert(part.as_str().to_string());
                }
                Rule::allergen => {
                    food.allergens.insert(part.as_str().to_string());
                }
                _ => panic!("Unexpected rule."),
            }
        }
        foods.push(food);
    }

    // Find all the allergens
    let mut all_allergens: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, food) in foods.iter().enumerate() {
        for allergen in &food.allergens {
            all_allergens
                .entry(allergen.to_string())
                .or_insert_with(Vec::new)
                .push(i)
        }
    }

    let mut dangerous_foods = HashSet::new();
    let mut possible_foods = HashMap::new();

    // For each allergen, we find the intersection of ingredients that can contain it.
    for (allergen, indexed_foods) in all_allergens {
        let mut foods_intersect = HashSet::new();
        for food in indexed_foods {
            let ingredients = foods.get(food).unwrap().ingredients.iter().collect();
            if foods_intersect.is_empty() {
                foods_intersect = ingredients;
            } else {
                foods_intersect = foods_intersect
                    .intersection(&ingredients)
                    .copied()
                    .collect();
            }
        }
        let mut possible = vec![];
        for food in foods_intersect {
            dangerous_foods.insert(food);
            possible.push(food);
        }
        possible_foods.insert(allergen, possible);
    }

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

    // Work out the dangerous ingredients.
    let mut dangerous: HashMap<String, String> = HashMap::new();
    let mut fixed = HashSet::new();

    loop {
        let mut count_fixed = 0;

        for (allergen, foods) in &possible_foods {
            if dangerous.contains_key(allergen) {
                continue;
            }

            let mut options = vec![];
            for f in foods {
                if !fixed.contains(*f) {
                    options.push(f);
                }
            }

            if options.len() == 1 {
                let bad_ingredient = **options.get(0).unwrap();
                fixed.insert(bad_ingredient.clone());
                dangerous.insert(allergen.clone(), bad_ingredient.to_string());
                count_fixed += 1;
            }
        }
        if count_fixed == 0 {
            break;
        }
    }

    let mut sorted_danger: Vec<(&String, &String)> = dangerous.iter().collect();
    sorted_danger.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    for (allergen, ingredient) in &sorted_danger {
        println!("{} in {}", allergen, ingredient);
    }
    let canonical_ingredients: Vec<_> = sorted_danger.iter().map(|(_, ingredient)| (*ingredient).to_string()).collect();
    let canonical_list = canonical_ingredients.join(",");
    (safe_uses, canonical_list)
}

#[derive(Debug)]
struct FoodStuff {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FoodStuff {
    fn new() -> Self {
        FoodStuff {
            ingredients: HashSet::new(),
            allergens: HashSet::new(),
        }
    }
}

#[derive(Parser)]
#[grammar = "parsers/day21.pest"]
pub struct DParser {}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_small() {
        let data_small = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!((5, String::from("mxmxvkd,sqjhc,fvjkl")), parse_foods(&data_small));
    }

    #[test]
    fn test_all() {
        assert_eq!((1679, String::from("lmxt,rggkbpj,mxf,gpxmf,nmtzlj,dlkxsxg,fvqg,dxzq")), parse_foods(data()));
    }
}
