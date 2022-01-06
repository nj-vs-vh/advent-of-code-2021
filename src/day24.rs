use super::utils::read_input;
use itertools::Itertools;
use memoize::memoize;
use std::collections::HashMap;
use std::{lazy::SyncLazy, sync::Mutex};

type Int = i64;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl Op {
    fn parse(s: &str) -> Option<Op> {
        match s {
            "add" => Some(Op::Add),
            "mul" => Some(Op::Mul),
            "div" => Some(Op::Div),
            "mod" => Some(Op::Mod),
            "eql" => Some(Op::Eql),
            &_ => None,
        }
    }

    fn result(&self, n1: &Int, n2: &Int) -> Int {
        match self {
            Op::Add => n1 + n2,
            Op::Mul => n1 * n2,
            Op::Div => n1 / n2,
            Op::Mod => n1 % n2,
            Op::Eql => {
                if n1 == n2 {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Var {
    X,
    Y,
    Z,
    W,
}

impl Var {
    fn parse(s: &str) -> Option<Var> {
        match s {
            "x" => Some(Var::X),
            "y" => Some(Var::Y),
            "z" => Some(Var::Z),
            "w" => Some(Var::W),
            &_ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Variable(Var),
    Number(Int),
}

impl Value {
    fn as_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Variable(var) => format!("{:?}", var),
        }
    }
}

type CodeBlock = Vec<(Op, Value, Value)>;

fn parse_code_blocks(input: &String) -> Vec<CodeBlock> {
    fn parse_expression(s: &str) -> Option<(Op, Value, Value)> {
        let mut tokens = s.split(' ');
        let op = Op::parse(tokens.next()?)?;
        let value1_str = tokens.next()?;
        let value1 = if let Some(var) = Var::parse(value1_str) {
            Value::Variable(var)
        } else {
            Value::Number(value1_str.parse().expect("Can't parse numberic value"))
        };
        let value2_str = tokens.next().unwrap();
        let value2 = if let Some(var) = Var::parse(value2_str) {
            Value::Variable(var)
        } else {
            Value::Number(value2_str.parse().expect("Can't parse numberic value"))
        };
        return Some((op, value1, value2));
    }

    let mut blocks: Vec<CodeBlock> = Vec::new();
    let mut current_block: CodeBlock = Vec::new();

    for line in input.lines() {
        if line.starts_with("inp") {
            if current_block.len() > 0 {
                blocks.push(current_block);
                current_block = Vec::new();
            }
            continue;
        }
        if let Some((o, v1, v2)) = parse_expression(line) {
            current_block.push((o, v1, v2));
        } else {
            println!("unable to parse line {}", line);
        }
    }
    if current_block.len() > 0 {
        blocks.push(current_block);
    }

    blocks
}

static CODE_BLOCKS: SyncLazy<Mutex<Vec<CodeBlock>>> = SyncLazy::new(|| Mutex::new(vec![]));

const DEBUG_EXECUTE: bool = false;

// #[memoize]
fn execute_code_block(code_block_idx: usize, z_init: Int, input: u8) -> Int {
    let mut var_values: HashMap<Var, Int> = HashMap::new();
    var_values.insert(Var::X, 0);
    var_values.insert(Var::Y, 0);
    var_values.insert(Var::Z, z_init);
    var_values.insert(Var::W, input as Int);

    if DEBUG_EXECUTE {
        println!(
            "\nexecuting code block\nx = {}, y = {}, z = {}, input = {}",
            var_values.get(&Var::X).unwrap(),
            var_values.get(&Var::Y).unwrap(),
            var_values.get(&Var::Z).unwrap(),
            var_values.get(&Var::W).unwrap()
        )
    }

    let code_blocks = CODE_BLOCKS.lock().unwrap();

    let code_block = code_blocks.get(code_block_idx).unwrap();

    for (op, v1, v2) in code_block {
        if DEBUG_EXECUTE {
            println!("{:?} {} {}", op, v1.as_string(), v2.as_string());
        }
        if let Value::Variable(var1) = v1 {
            let n1 = var_values.get(var1).unwrap();
            let n2 = match v2 {
                Value::Number(n) => n,
                Value::Variable(var2) => var_values.get(var2).unwrap(),
            };
            // set_var(*var1, op.result(&value1, &value2));
            let result = op.result(&n1, &n2);
            var_values.insert(*var1, result);
        } else {
            panic!();
        }
        if DEBUG_EXECUTE {
            println!(
                "x = {}, y = {}, z = {}",
                var_values.get(&Var::X).unwrap(),
                var_values.get(&Var::Y).unwrap(),
                var_values.get(&Var::Z).unwrap(),
            )
        }
    }

    *var_values.get(&Var::Z).unwrap()
}

#[memoize]
fn recurse(digit_idx: usize, z_value: Int) -> Option<Vec<u8>> {
    let code_blocks_total = CODE_BLOCKS.lock().unwrap().len();
    for digit_value in 1..=9 {
        let next_z_value = execute_code_block(digit_idx, z_value, digit_value);
        if digit_idx == code_blocks_total - 1 {
            if next_z_value == 0 {
                return Some(vec![digit_value]);
            }
        } else if let Some(mut valid_digits) = recurse(digit_idx + 1, next_z_value) {
            let mut res: Vec<u8> = Vec::new();
            res.push(digit_value);
            res.append(&mut valid_digits);
            return Some(res);
        }
    }
    None
}

fn find_valid_number() -> String {
    match recurse(0, 0) {
        None => "None".to_string(),
        Some(vec) => vec.iter().map(|d| d.to_string()).join(""),
    }
}

pub fn arithmetic_logic_unit() {
    let input = read_input(24, false);

    let blocks = parse_code_blocks(&input);

    // TEMP
    // let blocks = Vec::from_iter(blocks.into_iter().take(6));

    for b in blocks {
        CODE_BLOCKS.lock().unwrap().push(b);
    }

    println!("\n{}", find_valid_number());
}
