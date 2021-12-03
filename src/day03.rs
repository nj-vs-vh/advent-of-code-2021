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

fn filtering_rating(
    readings: &Vec<u32>,
    reading_len: usize,
    cmp: fn(f32, f32) -> bool,
    keep_if_equal: bool,
) -> u32 {
    let mut bit_counts: Vec<u32> = vec![0; reading_len];
    for reading in readings.iter() {
        for position in 0..reading_len {
            if bit_at(reading, position) {
                bit_counts[position] += 1;
            }
        }
    }

    let mut mask = vec![true; readings.len()];

    fn sum_mask(m: &Vec<bool>) -> u32 {
        m.iter().map(|b| *b as u32).sum::<u32>()
    }

    for filtering_position in (0..reading_len).rev() {
        if sum_mask(&mask) == 1 {
            for (i, m) in mask.iter().enumerate() {
                if *m {
                    return readings[i];
                }
            }
        }

        let filtering_bit_count = bit_counts[filtering_position] as f32;
        let half_unmasked: f32 = (sum_mask(&mask) as f32) / 2.0;
        let bit_to_keep = if filtering_bit_count == half_unmasked {
            keep_if_equal
        } else {
            cmp(filtering_bit_count, half_unmasked)
        };

        for (i, reading) in readings.iter().enumerate() {
            if mask[i] && bit_at(reading, filtering_position) != bit_to_keep {
                mask[i] = false;
                for position in 0..reading_len {
                    if bit_at(reading, position) {
                        bit_counts[position] -= 1;
                    }
                }
            }
        }
    }
    for (i, m) in mask.iter().enumerate() {
        if *m {
            return readings[i];
        }
    }
    panic!("Something's wrong!");
}

pub fn submarine_ratings() {
    let input = fs::read_to_string("data/day03/input.txt").expect("Can't read input file");
    let reading_len = input.lines().next().unwrap().len();
    let readings = Vec::from_iter(input.lines().map(parse_u32_from_binary));
    let readings_n = readings.len();

    // part 1
    let mut bit_counts: Vec<u32> = vec![0; reading_len];
    for reading in readings.iter() {
        for position in 0..reading_len {
            if bit_at(reading, position) {
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

    // part 2
    let o2_gen_rating = filtering_rating(
        &readings,
        reading_len,
        |bit_count, half_readings_count| bit_count > half_readings_count,
        true,
    );
    let co2_scrub_rating = filtering_rating(
        &readings,
        reading_len,
        |bit_count, half_readings_count| bit_count < half_readings_count,
        false,
    );
    println!(
        "oxygen generator rating: {}, CO2 scrubber rating: {}, life support rating: {}",
        o2_gen_rating,
        co2_scrub_rating,
        o2_gen_rating * co2_scrub_rating
    )
}
