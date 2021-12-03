use std::fs;

fn parse_u32_from_binary(s: &str) -> u32 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, ch)| match ch {
            '1' => u32::pow(2, i as u32),
            '0' => 0,
            _ => panic!("Encountered non-binary char!"),
        })
        .sum()
}

fn bit_at(input: &u32, n: usize) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

pub fn gamma_epsilon_rates() {
    let input = fs::read_to_string("data/day03/input.txt").expect("Can't read input file");
    let reading_len = input.lines().next().unwrap().len();
    let readings = Vec::from_iter(input.lines().map(parse_u32_from_binary));
    let readings_n = readings.len();

    let mut bit_counts: Vec<u32> = vec![0; reading_len];
    for reading in readings {
        for position in 0..reading_len {
            if bit_at(&reading, position) {
                bit_counts[position] += 1;
            }
        }
    }

    let gamma = parse_u32_from_binary(&String::from_iter(bit_counts.iter().rev().map(|bc| {
        if bc > &((readings_n / 2) as u32) {
            '1'
        } else {
            '0'
        }
    })));
    let epsilon = !gamma % u32::pow(2, reading_len as u32); // dropping bytes higher than reading_len
    println!(
        "gamma = {:#0b}, epsilon = {:#0b}, power consumption = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
