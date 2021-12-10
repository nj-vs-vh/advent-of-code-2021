use super::utils::read_input;

#[derive(PartialEq)]
enum BracketType {
    Regular,
    Square,
    Curly,
    Angle,
}

impl BracketType {
    fn corrupted_score(&self) -> u32 {
        match self {
            BracketType::Regular => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }
    fn incomplete_score(&self) -> u64 {
        match self {
            BracketType::Regular => 1,
            BracketType::Square => 2,
            BracketType::Curly => 3,
            BracketType::Angle => 4,
        }
    }
}

enum BracketRole {
    Opening,
    Closing,
}

struct Bracket {
    role: BracketRole,
    type_: BracketType,
}

impl Bracket {
    fn parse(c: &char) -> Bracket {
        match c {
            '(' => Bracket {
                role: BracketRole::Opening,
                type_: BracketType::Regular,
            },
            '[' => Bracket {
                role: BracketRole::Opening,
                type_: BracketType::Square,
            },
            '{' => Bracket {
                role: BracketRole::Opening,
                type_: BracketType::Curly,
            },
            '<' => Bracket {
                role: BracketRole::Opening,
                type_: BracketType::Angle,
            },
            ')' => Bracket {
                role: BracketRole::Closing,
                type_: BracketType::Regular,
            },
            ']' => Bracket {
                role: BracketRole::Closing,
                type_: BracketType::Square,
            },
            '}' => Bracket {
                role: BracketRole::Closing,
                type_: BracketType::Curly,
            },
            '>' => Bracket {
                role: BracketRole::Closing,
                type_: BracketType::Angle,
            },
            _ => {
                panic!("Can't parse bracket!")
            }
        }
    }
}

pub fn brackets_parsing() {
    let input = read_input(10, false);

    let mut corrupted_lines_score: u32 = 0; // pt 1
    let mut incomplete_line_scores: Vec<u64> = Vec::new(); // pt 2
    for line in input.lines() {
        let brackets: Vec<Bracket> = line.chars().map(|c| Bracket::parse(&c)).collect();
        let mut stack: Vec<BracketType> = Vec::new();

        let mut is_corrupted = false;
        for b in brackets {
            match b.role {
                BracketRole::Opening => stack.push(b.type_),
                BracketRole::Closing => match stack.pop() {
                    Some(type_) => {
                        if type_ != b.type_ {
                            corrupted_lines_score += b.type_.corrupted_score();
                            is_corrupted = true;
                            break;
                        }
                    }
                    None => {
                        panic!("We can't handle unopened brackets yet!")
                    }
                },
            }
        }
        if !is_corrupted && stack.len() > 0 {
            let mut incomplete_line_score: u64 = 0;
            for b in stack.iter().rev() {
                incomplete_line_score *= 5;
                incomplete_line_score += b.incomplete_score();
            }
            incomplete_line_scores.push(incomplete_line_score);
        }
    }
    println!(
        "Total score for corrupted lines is {}",
        corrupted_lines_score
    );
    incomplete_line_scores.sort();
    println!(
        "Incomplete line scores: {:?}",
        incomplete_line_scores[incomplete_line_scores.len() / 2]
    );
}
