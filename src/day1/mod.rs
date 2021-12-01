use std::fs;

fn count_increases(v: &Vec<u32>) -> u32 {
    v.windows(2).map(|p| if p[1] > p[0] { 1 } else { 0 }).sum()
}

pub fn sonar() {
    let mut depths: Vec<u32> = Vec::new();
    let input = fs::read_to_string("data/day1/input.txt").expect("Cannot read file!");
    for line in input.lines() {
        let depth: u32 = line.parse().expect("Cannot parse u32 from line");
        depths.push(depth);
    }

    // task 1
    println!("number of increasing depths: {}", count_increases(&depths));

    // task 2
    let depths_ma = Vec::from_iter(depths.windows(3).map(|ds| ds.iter().sum()));
    println!(
        "number of increasing depths (3-steps moving average): {}",
        count_increases(&depths_ma)
    );
}
