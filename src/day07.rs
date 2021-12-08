use std::fs;

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

    let mut fuel_costs: Vec<i32> = Vec::with_capacity((max_position - min_position) as usize);
    let mut latest_fuel_cost: i32 = positions.iter().map(|x| (x - min_position).abs()).sum();
    fuel_costs.push(latest_fuel_cost);
    for align_to in (min_position + 1)..=max_position {
        for p in &positions {
            if p < &align_to {
                latest_fuel_cost += 1
            } else {
                latest_fuel_cost -= 1
            }
        }
        fuel_costs.push(latest_fuel_cost);
    }
    println!(
        "if crabs spend fuel uniformly: {}",
        fuel_costs.iter().min().unwrap()
    );

    // part 2

    fn sum_range(top: i32) -> i32 {
        (0..=top).sum()
    }

    let mut fuel_costs: Vec<i32> = Vec::with_capacity((max_position - min_position) as usize);
    let mut latest_fuel_cost: i32 = positions
        .iter()
        .map(|x| sum_range((x - min_position).abs()))
        .sum();
    fuel_costs.push(latest_fuel_cost);
    for align_to in (min_position + 1)..=max_position {
        for p in &positions {
            if p < &align_to {
                latest_fuel_cost += (p - align_to).abs();
            } else {
                latest_fuel_cost -= (p - align_to + 1).abs();
            }
        }
        fuel_costs.push(latest_fuel_cost);
    }

    println!(
        "if crabs spend more fuel each step: {}",
        fuel_costs.iter().min().unwrap()
    );
}
