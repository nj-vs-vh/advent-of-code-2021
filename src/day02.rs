use std::fs;

#[derive(Debug)]
enum SubmarineMove {
    Up(u16),
    Forward(u16),
    Down(u16),
}

impl SubmarineMove {
    fn parse(line: &str) -> SubmarineMove {
        let mut line_parts = line.split_whitespace();
        let direction = line_parts.next().expect("Can't get direction from line");
        let amount: u16 = line_parts
            .next()
            .expect("Can't get move amount from line")
            .parse()
            .expect("Can't parse move amount as u32");
        match direction {
            "up" => SubmarineMove::Up(amount),
            "forward" => SubmarineMove::Forward(amount),
            "down" => SubmarineMove::Down(amount),
            &_ => {
                panic!("can't parse submarine move from line {}", line)
            }
        }
    }

    fn as_deltas_wo_aim(&self) -> (i32, i32) {
        match self {
            SubmarineMove::Down(a) => (0, *a as i32),
            SubmarineMove::Up(a) => (0, -1 * (*a as i32)),
            SubmarineMove::Forward(a) => (*a as i32, 0),
        }
    }

    fn perform(&self, curr_horiz: i32, curr_depth: i32, curr_aim: i32) -> (i32, i32, i32) {
        match self {
            SubmarineMove::Down(plus_aim) => {
                (curr_horiz, curr_depth, curr_aim + (*plus_aim as i32))
            }
            SubmarineMove::Up(minus_aim) => {
                (curr_horiz, curr_depth, curr_aim - (*minus_aim as i32))
            }
            SubmarineMove::Forward(forward) => (
                curr_horiz + (*forward as i32),
                curr_depth + curr_aim * (*forward as i32),
                curr_aim,
            ),
        }
    }
}

pub fn submarine_path() {
    let input = fs::read_to_string("data/day02/input.txt").expect("Cannot read file!");
    let moves = Vec::from_iter(input.lines().map(SubmarineMove::parse));

    // part 1
    let (horiz, depth) = moves
        .iter()
        .map(SubmarineMove::as_deltas_wo_aim)
        .fold((0, 0), |acc, deltas| (acc.0 + deltas.0, acc.1 + deltas.1));

    println!(
        "[part1] submarine will be at ({}, {}), product is: {}",
        horiz,
        depth,
        horiz * depth
    );

    let (horiz, depth, aim) = moves.iter().fold((0, 0, 0), |acc, submarine_move| {
        submarine_move.perform(acc.0, acc.1, acc.2)
    });
    println!(
        "[part2] submarine will be at ({}, {}), aimed at {}, product is: {}",
        horiz,
        depth,
        aim,
        horiz * depth
    );
}
