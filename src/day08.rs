use std::collections::HashMap;
use std::fs;

const N_SEGMENTS: usize = 7;
const N_DIGITS: usize = 10;
const DIGITS: &str = "abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg";

fn ch2i(ch: char) -> usize {
    ch as usize - 'a' as usize
}

fn count_segment_occurrences(characters: &str) -> [u32; N_SEGMENTS] {
    let mut res: [u32; N_SEGMENTS] = [0; N_SEGMENTS];
    for digit in characters.split(' ') {
        for ch in digit.chars() {
            res[ch2i(ch)] += 1;
        }
    }
    res
}

#[derive(Hash, PartialEq, Eq)]
struct OccurenceBasedHash {
    occurence_frequencies: [u32; N_DIGITS],
}

impl OccurenceBasedHash {
    fn calculate(digit: &str, segment_occurences: [u32; N_SEGMENTS]) -> OccurenceBasedHash {
        // how many segments with each occurence frequency constitute the unknown symbol?
        // frequencies in principle range from 0 to N_DIGITS (every digit has this segment)
        let mut res: [u32; N_DIGITS] = [0; N_DIGITS];
        for ch in digit.chars() {
            res[segment_occurences[ch2i(ch)] as usize] += 1
        }
        OccurenceBasedHash {
            occurence_frequencies: res,
        }
    }
}

pub fn messed_up_displays() {
    let input = fs::read_to_string("data/day08/input.txt").expect("Can't read input file");

    let mut digit_by_ob_hash: HashMap<OccurenceBasedHash, usize> = HashMap::new();
    let segment_occurrences_in_digits = count_segment_occurrences(DIGITS);
    for (i, d) in DIGITS.split(' ').enumerate() {
        digit_by_ob_hash.insert(
            OccurenceBasedHash::calculate(d, segment_occurrences_in_digits),
            i,
        );
    }

    let mut sum_of_readings: u32 = 0;
    for entry in input.lines() {
        let messed_up_digits = entry.split(" | ").next().expect("Can't get digits");
        let segment_occurences = count_segment_occurrences(messed_up_digits);

        let messed_up_readings = entry.split(" | ").nth(1).expect("Can't get reading");
        let readings: Vec<&usize> = messed_up_readings
            .split(' ')
            .map(|mur| {
                digit_by_ob_hash
                    .get(&OccurenceBasedHash::calculate(mur, segment_occurences))
                    .unwrap()
            })
            .collect();
        let reading: u32 = readings
            .iter()
            .rev()
            .enumerate()
            .map(|(i, r)| (**r as u32) * (10 as u32).pow(i as u32))
            .sum();

        sum_of_readings += reading;
    }
    println!("{:?}", sum_of_readings);
}
