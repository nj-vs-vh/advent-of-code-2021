use std::fs;

fn count_increases(v: &Vec<u32>) -> u32 {
    let mut n_increases: u32 = 0;
    for pair in v.windows(2) {
        if pair[1] > pair[0] {
            n_increases += 1;
        }
    }
    n_increases
}

pub fn sonar() {
    let input = fs::read_to_string("data/day1/input.txt").expect("Cannot read file!");

    let mut depths: Vec<u32> = Vec::new();

    for line in input.lines() {
        let depth: u32 = line.parse().expect("Cannot parse u32 from line");
        depths.push(depth);
    }

    // task 1
    println!("number of increasing depths: {}", count_increases(&depths));

    // task 2
    let depths_ma = Vec::from_iter(depths.windows(3).map(|ds| (ds[0] + ds[1] + ds[2]) / 3));
    println!(
        "number of increasing depths (3-steps moving average): {}",
        count_increases(&depths_ma)
    );
}
