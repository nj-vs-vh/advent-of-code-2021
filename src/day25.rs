use super::utils::read_input;
use std::{thread, time};

#[derive(PartialEq, Eq)]
enum CellState {
    Empty,
    Right,
    Down,
}

impl CellState {
    fn parse(c: &char) -> CellState {
        match c {
            '>' => CellState::Right,
            'v' => CellState::Down,
            '.' => CellState::Empty,
            _ => {
                panic!("Can't parse cell state!");
            }
        }
    }
}

type Map = Vec<Vec<CellState>>;

fn parse_map(s: String) -> Map {
    let mut map: Map = Vec::new();
    for line in s.lines() {
        let mut row: Vec<CellState> = Vec::new();
        for c in line.chars() {
            row.push(CellState::parse(&c))
        }
        map.push(row);
    }
    map
}

fn step(map: &mut Map) -> usize {
    let i_len = map.len();
    let j_len = map.iter().next().unwrap().len();

    let mut total_step_count: usize = 0;
    // pass for right-facing cucumbers
    let mut stepping_idx: Vec<(usize, usize)> = Vec::new();
    for i in 0..i_len {
        for j in 0..j_len {
            let j_pre = if j > 0 { j - 1 } else { j_len - 1 };
            if map[i][j] == CellState::Empty && map[i][j_pre] == CellState::Right {
                stepping_idx.push((i, j_pre));
            }
        }
    }
    total_step_count += stepping_idx.len();
    for (i, j) in stepping_idx {
        let j_next = (j + 1) % j_len;
        map[i][j] = CellState::Empty;
        map[i][j_next] = CellState::Right;
    }
    // pass for down-facing cucumbers
    stepping_idx = Vec::new();
    for i in 0..i_len {
        for j in 0..j_len {
            let i_next = (i + 1) % i_len;
            if map[i][j] == CellState::Down && map[i_next][j] == CellState::Empty {
                stepping_idx.push((i, j));
            }
        }
    }
    total_step_count += stepping_idx.len();
    for (i, j) in stepping_idx {
        let i_next = (i + 1) % i_len;
        map[i][j] = CellState::Empty;
        map[i_next][j] = CellState::Down;
    }

    total_step_count
}

fn print_map(map: &Map) {
    let i_len = map.len();
    let j_len = map.iter().next().unwrap().len();
    for i in 0..i_len {
        for j in 0..j_len {
            print!(
                "{}",
                match map[i][j] {
                    CellState::Empty => '.',
                    CellState::Right => '>',
                    CellState::Down => 'v',
                }
            )
        }
        println!();
    }
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

const DEBUG: bool = false;

pub fn cucumbers_drift() {
    let input = read_input(25, false);

    let mut map = parse_map(input);

    let sleep_dur = time::Duration::from_millis(50);

    let mut counter: usize = 1;
    while step(&mut map) > 0 {
        counter += 1;
        if DEBUG {
            clear_terminal();
            print_map(&map);
            thread::sleep(sleep_dur);
        }
    }

    println!("\n{}", counter);
}
