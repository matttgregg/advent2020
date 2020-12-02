use std::collections::HashSet;
use std::error::Error;
use std::fs;

pub fn run(config: advent2020::Config) -> Result<(), Box<dyn Error>> {
    run_file(&config.filename)
}

pub fn run_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let contents: HashSet<i32> = contents
        .lines()
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect();
    println!("Scanning {} entries.", contents.len());

    // Find sums to 2020
    let part1 = sums_to(&contents, 2020);
    for (a, b) in part1.iter() {
        println!("{} x {} -> {}", a, b, a * b);
    }

    let part2 = triple_sums_to(&contents, 2020);
    for (a, b, c) in part2.iter() {
        println!("{} x {} x {} -> {}", a, b, c, a * b * c);
    }

    Ok(())
}

fn sums_to(vals: &HashSet<i32>, total: i32) -> Vec<(i32, i32)> {
    let mut results: Vec<(i32, i32)> = Vec::new();
    for val in vals.iter() {
        let complement = total - val;
        if val < &complement && vals.contains(&complement) {
            results.push((*val, complement));
        }
    }
    results
}

fn triple_sums_to(vals: &HashSet<i32>, total: i32) -> Vec<(i32, i32, i32)> {
    let mut results: Vec<(i32, i32, i32)> = Vec::new();
    for val in vals.iter() {
        let complement = total - val;
        if val < &complement {
            let complement_sums = sums_to(vals, complement);
            for (a, b) in complement_sums.iter() {
                if val < a {
                    results.push((*val, *a, *b));
                }
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let data = vec![1721, 979, 366, 299, 675, 1456];
        let data: HashSet<i32> = data.into_iter().collect();
        let res = sums_to(&data, 2020);
        assert_eq!(1, res.len());
        assert_eq!((299, 1721), res[0]);
    }

    #[test]
    fn part2_test() {
        let data = vec![1721, 979, 366, 299, 675, 1456];
        let data: HashSet<i32> = data.into_iter().collect();
        let res = triple_sums_to(&data, 2020);
        assert_eq!(1, res.len());
        assert_eq!((366, 675, 979), res[0]);
    }
}
