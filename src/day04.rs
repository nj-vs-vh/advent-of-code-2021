use std::convert::TryInto;
use std::fs;

const BOARD_SIDE: usize = 5;

#[derive(Debug)]
struct Board {
    values: [u32; BOARD_SIDE * BOARD_SIDE],
    marked: [bool; BOARD_SIDE * BOARD_SIDE],
}

impl Board {
    fn _flat_idx(i: usize, j: usize) -> Option<usize> {
        if !(i < BOARD_SIDE && j < BOARD_SIDE) {
            return None;
        }
        return Some(i * BOARD_SIDE + j);
    }

    fn print(&self) {
        for i in 0..BOARD_SIDE {
            for j in 0..BOARD_SIDE {
                let is_marked = self.is_marked(i, j).unwrap();
                print!(
                    " {}{:#2}{} ",
                    if is_marked { '[' } else { ' ' },
                    self.get(i, j).unwrap(),
                    if is_marked { ']' } else { ' ' }
                );
            }
            print!("\n");
        }
    }

    fn get(&self, i: usize, j: usize) -> Option<u32> {
        match Board::_flat_idx(i, j) {
            None => None,
            Some(fi) => Some(self.values[fi]),
        }
    }

    fn is_marked(&self, i: usize, j: usize) -> Option<bool> {
        match Board::_flat_idx(i, j) {
            None => None,
            Some(fi) => Some(self.marked[fi]),
        }
    }

    fn is_won(&self) -> bool {
        for i in 0..BOARD_SIDE {
            let mut all_marked_in_row = true;
            let mut all_marked_in_col = true;
            for j in 0..BOARD_SIDE {
                if !self.is_marked(i, j).unwrap() {
                    all_marked_in_row = false;
                }
                if !self.is_marked(j, i).unwrap() {
                    all_marked_in_col = false;
                }
            }
            if all_marked_in_row || all_marked_in_col {
                return true;
            }
        }
        return false;
    }

    fn unmarked(&self) -> impl Iterator<Item = u32> + '_ {
        self.values
            .iter()
            .zip(self.marked.iter())
            .filter(|&(_value, &marked)| !marked)
            .map(|(&value, _marked)| value)
    }

    fn score(&self, last_drawn: &u32) -> u32 {
        self.unmarked().sum::<u32>() * last_drawn
    }

    fn parse(s: &str) -> Board {
        let values_vec: Vec<u32> = s
            .replace("\n", " ")
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse().expect("Cannot parse integer"))
            .collect();
        Board {
            values: values_vec.try_into().expect("Size mismatch"),
            marked: [false; BOARD_SIDE * BOARD_SIDE],
        }
    }

    fn mark(&mut self, marked_value: &u32) {
        for (fi, value) in self.values.iter().enumerate() {
            if value == marked_value {
                self.marked[fi] = true;
            }
        }
    }
}

pub fn squid() {
    let input = fs::read_to_string("data/day04/input.txt").expect("Can't read input file");
    let mut input_blocks = input.split("\n\n");

    let drawn_numbers: Vec<u32> = input_blocks
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().expect("Can't parse drawn number as u32"))
        .collect();

    let mut boards: Vec<Board> = input_blocks.map(Board::parse).collect();
    let mut won_boards = vec![false; boards.len()];
    let total_board_count = boards.len();

    println!("drawing numbers");
    for drawn_number in drawn_numbers {
        print!("{} ", drawn_number);
        for (board_idx, board) in boards.iter_mut().enumerate() {
            board.mark(&drawn_number);
            if board.is_won() {
                won_boards[board_idx] = true;
                let won_boards_count: usize = won_boards.iter().map(|&b| b as usize).sum();
                if won_boards_count == 1 {
                    println!("\n\nboard {} has won first", board_idx);
                    board.print();
                    println!("score = {}\n", board.score(&drawn_number));
                }
                if won_boards_count == total_board_count {
                    println!("\n\nboard {} has won last", board_idx);
                    board.print();
                    println!("score = {}\n", board.score(&drawn_number));
                    return;
                }
            }
        }
    }
}
