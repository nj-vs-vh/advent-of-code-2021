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

fn count_bits(readings: &Vec<u32>, reading_len: usize) -> Vec<u32> {
    let mut bit_counts: Vec<u32> = vec![0; reading_len];
    for reading in readings.iter() {
        for position in 0..reading_len {
            if bit_at(reading, position) {
                bit_counts[position] += 1;
            }
        }
    }
    bit_counts
}

fn filtering_rating(
    readings: &Vec<u32>,
    reading_len: usize,
    cmp: fn(f32, f32) -> bool,
    keep_if_equal: bool,
) -> u32 {
    let mut bit_counts = count_bits(readings, reading_len);

    let mut mask = vec![true; readings.len()];
    let mut mask_sum = readings.len();

    fn get_true_idx(m: &Vec<bool>) -> Option<usize> {
        for (i, m) in m.iter().enumerate() {
            if *m {
                return Some(i);
            }
        }
        return None;
    }

    for filtering_position in (0..reading_len).rev() {
        if mask_sum == 1 {
            return readings[get_true_idx(&mask).unwrap()];
        }

        let ones_at_filtering_bit = bit_counts[filtering_position] as f32;
        let half_readings_unmasked: f32 = (mask_sum as f32) / 2.0;

        let bit_to_keep = if ones_at_filtering_bit == half_readings_unmasked {
            keep_if_equal
        } else {
            cmp(ones_at_filtering_bit, half_readings_unmasked)
        };

        for (i, reading) in readings.iter().enumerate() {
            if mask[i] && bit_at(reading, filtering_position) != bit_to_keep {
                mask[i] = false;
                mask_sum -= 1;
                for position in 0..reading_len {
                    if bit_at(reading, position) {
                        bit_counts[position] -= 1;
                    }
                }
            }
        }
    }
    return readings[get_true_idx(&mask).unwrap()];
}

pub fn submarine_ratings() {
    let input = fs::read_to_string("data/day03/input.txt").expect("Can't read input file");
    let reading_len = input.lines().next().unwrap().len();
    let readings = Vec::from_iter(input.lines().map(parse_u32_from_binary));
    let readings_n = readings.len();

    // part 1
    let bit_counts = count_bits(&readings, reading_len);

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
