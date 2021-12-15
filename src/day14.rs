use super::utils::read_input;
use std::collections::HashMap;

fn char_windows<'a>(src: &'a String, win_size: usize) -> impl Iterator<Item = &'a str> {
    // credit: https://stackoverflow.com/a/51261570
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

const N_CHAR: usize = 26;

fn ch2idx(ch: char) -> usize {
    ch as usize - 'A' as usize
}

fn idx2ch(i: usize) -> char {
    (('A' as usize + i) as u8) as char
}

fn character_counts(
    pair_table: &[[i64; N_CHAR]; N_CHAR],
    intial_polymer: &String,
) -> HashMap<char, i64> {
    let mut counts: HashMap<char, i64> = HashMap::new();
    let mut ch: char;
    let mut count: i64;
    let last_char = intial_polymer.chars().last().unwrap(); // last char is unchanged
    for i_left in 0..N_CHAR {
        ch = idx2ch(i_left);
        count = 0;
        if ch == last_char {
            count = 1;
        }
        for i_right in 0..N_CHAR {
            count += pair_table[i_left][i_right];
        }
        if count > 0 {
            counts.insert(ch, count);
        }
    }
    counts
}

pub fn polymers() {
    let input = read_input(14, false);

    let mut insertion_rules: HashMap<(usize, usize), usize> = HashMap::new();
    for line in input.lines().skip(2) {
        let mut line_parts = line.split(" -> ");
        let between = line_parts.next().unwrap();
        let ch_left = between.chars().nth(0).unwrap();
        let ch_right = between.chars().nth(1).unwrap();
        let insert = line_parts.next().unwrap().chars().next().unwrap();
        insertion_rules.insert((ch2idx(ch_left), ch2idx(ch_right)), ch2idx(insert));
    }

    // creating a table of numbers of character pairs
    let mut pair_table: [[i64; N_CHAR]; N_CHAR] = [[0; N_CHAR]; N_CHAR];
    let initial_polymer = input.lines().next().unwrap().to_string();
    for pair in char_windows(&initial_polymer, 2) {
        let ch_left = pair.chars().nth(0).unwrap();
        let ch_right = pair.chars().nth(1).unwrap();
        pair_table[ch2idx(ch_left)][ch2idx(ch_right)] += 1;
    }

    let mut pair_table_update: [[i64; N_CHAR]; N_CHAR];

    let iterations: usize = 40;
    let mut n_pairs: i64;
    for _ in 0..iterations {
        pair_table_update = [[0; N_CHAR]; N_CHAR];
        for i_left in 0..N_CHAR {
            for i_right in 0..N_CHAR {
                n_pairs = pair_table[i_left][i_right];
                if n_pairs == 0 {
                    continue;
                }
                if let Some(i_insert) = insertion_rules.get(&(i_left, i_right)) {
                    pair_table_update[i_left][i_right] -= n_pairs;
                    pair_table_update[i_left][*i_insert] += n_pairs;
                    pair_table_update[*i_insert][i_right] += n_pairs;
                }
            }
        }
        for i_left in 0..N_CHAR {
            for i_right in 0..N_CHAR {
                pair_table[i_left][i_right] += pair_table_update[i_left][i_right];
            }
        }
    }

    let counts = character_counts(&pair_table, &initial_polymer);
    let max_count = counts.values().max().unwrap();
    let min_count = counts.values().min().unwrap();
    println!(
        "Most common N - least common N after {} iterations: {}",
        iterations,
        max_count - min_count
    );
}
