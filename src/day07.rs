use std::fs;

fn fuel_cost_pt1(a: i32, positions: &Vec<i32>) -> i32 {
    positions.iter().map(|x| (x - a).abs()).sum()
}

fn fuel_cost_pt2(a: i32, positions: &Vec<i32>) -> i32 {
    fn sum_range(top: i32) -> i32 {
        (0..=top).sum()
    }

    positions.iter().map(|x| sum_range((x - a).abs())).sum()
}

pub fn align_crabs() {
    let input = fs::read_to_string("data/day07/input.txt").expect("Can't read input file");
    let positions: Vec<i32> = input
        .lines()
        .next()
        .expect("Input must contain at least one line")
        .split(',')
        .map(|s| s.parse().expect("Can't parse integer"))
        .collect();
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();

    // part 1
    let optimal_position = (min_position..=max_position)
        .map(|p| fuel_cost_pt1(p, &positions))
        .min()
        .unwrap();

    println!("if crabs spend fuel uniformly: {}", optimal_position);

    // part 2
    let optimal_position = (min_position..=max_position)
        .map(|p| fuel_cost_pt2(p, &positions))
        .min()
        .unwrap();

    println!("if crabs spend more fuel each step: {}", optimal_position);
}
