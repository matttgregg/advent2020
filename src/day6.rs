use std::time::SystemTime;

pub fn run() {
    println!("Day6!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data6.txt");
    let contents = String::from_utf8_lossy(cbytes);

    let (any, all) = run_groups_all(&contents);
    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();

    println!("Groups sum (ANY): {}", any);
    println!("Groups sum (ALL): {}", all);
    println!("Timed: {}us", timed);
}

pub fn run_groups_all(data: &str) -> (u32, u32) {
    let mut groups_sum_all = 0;
    let mut groups_sum_any = 0;
    let mut filter_all = u32::MAX;
    let mut filter_any = 0_u32;
    for line in data.lines() {
        let mut ans_bin = 0_u32;
        for c in line.chars() {
            ans_bin |= 2_u32.pow(u32::from(c) - u32::from('a'));
        }

        if ans_bin == 0 {
            groups_sum_all += filter_all.count_ones();
            groups_sum_any += filter_any.count_ones();
            filter_any = 0;
            filter_all = u32::MAX;
        } else {
            filter_any |= ans_bin;
            filter_all &= ans_bin;
        }
    }

    // Collect final values.
    groups_sum_all += filter_all.count_ones();
    groups_sum_any += filter_any.count_ones();
    (groups_sum_any, groups_sum_all)
}
