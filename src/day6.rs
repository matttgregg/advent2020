use std::time::SystemTime;

use std::collections::HashSet;

pub fn run() {
    println!("Day5!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data6.txt");
    let contents = String::from_utf8_lossy(cbytes);

    let res = run_groups_any(&contents);
    let res2 = run_groups_all(&contents);
    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();

    println!("Groups sum (ANY): {}", res);
    println!("Groups sum (ALL): {}", res2);
    println!("Timed: {}us", timed);
}

pub fn run_groups_any(data: &str) -> usize  {
    let mut group_data = HashSet::new();
    let mut groups_sum = 0;
    for line in data.lines() {
        let mut char_count = 0;
        for c in line.chars() {
            char_count += 1;
            group_data.insert(c);
        }
        if char_count == 0 {
            groups_sum += group_data.len();
            group_data.clear();
        }
    }

    // Collect final values.
    groups_sum += group_data.len();
    group_data.clear();
    groups_sum
}

pub fn run_groups_all(data: &str) -> usize  {
    let mut group_data = HashSet::new();
    let mut groups_sum = 0;
    let mut group_i = 0;
    for line in data.lines() {
        let mut char_count = 0;
        let mut new_group_data = HashSet::new();
        for c in line.chars() {
            char_count += 1;
            if group_i == 0 || group_data.contains(&c) {
                // We only collect chars collected by the previous passenger.
                new_group_data.insert(c);
            }
        }

        if char_count == 0 {
            groups_sum += group_data.len();
            group_data.clear();
            group_i = 0;
        } else {
            group_data = new_group_data;
            group_i += 1;
        }
    }

    // Collect final values.
    groups_sum += group_data.len();
    group_data.clear();
    groups_sum
}
