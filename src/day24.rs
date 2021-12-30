use super::utils::read_input;
use std::collections::{HashMap, HashSet};
use std::iter;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl Op {
    fn parse(s: &str) -> Option<Op> {
        match s {
            "inp" => Some(Op::Inp),
            "add" => Some(Op::Add),
            "mul" => Some(Op::Mul),
            "div" => Some(Op::Div),
            "mod" => Some(Op::Mod),
            "eql" => Some(Op::Eql),
            &_ => None,
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

type Value2Digits = HashMap<i32, HashSet<u8>>; // {numeric value: {set of digits resulting in this input}
type InputMapping = HashMap<usize, Value2Digits>; // {idx in input: Value2Digits @ this idx}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Variable(Var),
    Number(i32),
    InputMapping(InputMapping), // {idx in input: {numeric value: {set of digits in this input}}}
}

impl Value {
    fn print(&self) {
        fn print_recursive(v: &Value, indent: usize) {
            let indentation: String = iter::repeat(' ').take(indent).collect();
            match v {
                Value::Number(n) => println!("{}{}", indentation, n),
                Value::Variable(v) => println!("{}{:?}", indentation, v),
                // Value::InputDigit(idx) => println!("{}input[{}]", indentation, idx),
                // Value::Expression(op, operands) => {
                //     println!("{}{:?}", indentation, op);
                //     for v in operands {
                //         print_recursive(v, indent + 4);
                //     }
                // }
                // Value::FromInputDigitMapping(map) => {
                //     println!("{:?}", map);
                // }
                Value::InputMapping(im) => println!("{:?}", im),
            }
        }

        print_recursive(self, 0)
    }
}

fn parse_expression(s: &str, digit_idx: usize) -> (Op, Value, Value) {
    let mut tokens = s.split(' ');
    let op = Op::parse(tokens.next().unwrap()).unwrap();
    let value1_str = tokens.next().unwrap();
    let value1 = if let Some(var) = Var::parse(value1_str) {
        Value::Variable(var)
    } else {
        Value::Number(value1_str.parse().expect("Can't parse numberic value"))
    };
    if op == Op::Inp {
        let mut i2v: InputMapping = HashMap::new();
        let mut tirivial_map: Value2Digits = HashMap::new();
        for digit in 0..10 {
            let mut trivial_set = HashSet::new();
            trivial_set.insert(digit);
            tirivial_map.insert(digit as i32, trivial_set);
        }
        i2v.insert(digit_idx, tirivial_map);
        return (op, value1, Value::InputMapping(i2v));
    }
    let value2_str = tokens.next().unwrap();
    let value2 = if let Some(var) = Var::parse(value2_str) {
        Value::Variable(var)
    } else {
        Value::Number(value2_str.parse().expect("Can't parse numberic value"))
    };
    return (op, value1, value2);
}

type VariableValues = HashMap<Var, Value>;

fn perform_operation(op: &Op, v1: &Value, v2: &Value, var_values: &VariableValues) -> Value {
    let v1 = if let Value::Variable(var) = v1 {
        var_values.get(var).unwrap()
    } else {
        v1
    };
    let v2 = if let Value::Variable(var) = v2 {
        var_values.get(var).unwrap()
    } else {
        v2
    };

    match op {
        Op::Inp => v2.clone(),
        op => {
            fn short_circuit(op: &Op, value: &Value, other_value: &Value) -> Option<Value> {
                if let Value::Number(n) = value {
                    if *n == 0 {
                        match op {
                            Op::Mul | Op::Mod | Op::Div => {
                                return Some(Value::Number(0));
                            }
                            Op::Add => {
                                return Some(other_value.clone());
                            }
                            _ => {}
                        }
                    } else if *n == 1 && op == &Op::Mul {
                        return Some(other_value.clone());
                    }
                }
                None
            }

            if let Some(value) = short_circuit(op, v1, v2) {
                return value;
            }
            if let Some(value) = short_circuit(op, v2, v1) {
                return value;
            }

            if let (Value::Number(n1), Value::Number(n2)) = (v1, v2) {
                return match op {
                    // TODO: move this to Op method
                    Op::Add => Value::Number(n1 + n2),
                    Op::Mul => Value::Number(n1 * n2),
                    Op::Div => Value::Number(n1 / n2),
                    Op::Mod => Value::Number(n1 % n2),
                    Op::Eql => Value::Number(if n1 == n2 { 1 } else { 0 }),
                    Op::Inp => {
                        panic!("Input statement is already covered in outer match")
                    }
                };
            } else {
                // if one is a mapping and the other is a number -- apply operation to all mapping values
                // if both are mappings for the same digit index -- merge them (cartesian product, intersecting digit value sets)
                // otherwise -- return Expression with two of them
                panic!();
            }
        }
    }
}

const N_INPUTS: usize = 1;

pub fn arithmetic_logic_unit() {
    let input = read_input(24, true);

    let mut var_values: VariableValues = HashMap::new();
    var_values.insert(Var::X, Value::Number(0));
    var_values.insert(Var::Y, Value::Number(0));
    var_values.insert(Var::Z, Value::Number(0));
    var_values.insert(Var::W, Value::Number(0));

    // let input_digits: [i32; N_INPUTS] = [1];
    let mut next_digit: usize = 0;
    for (i, line) in input.lines().enumerate() {
        println!("{}. {}", i, line);
        let (op, v1, v2) = parse_expression(line, next_digit);
        if op == Op::Inp {
            next_digit += 1;
        }
        let assign_to = &v1;
        v2.print();
        if let Value::Variable(assign_to) = assign_to {
            let result = perform_operation(&op, &v1, &v2, &var_values);
            var_values.insert(*assign_to, result);
        } else {
            panic!("Expecting the first operand to be a variable to assign result to")
        }
    }

    for (var, value) in &var_values {
        print!("\n{:?} = ", var);
        value.print();
    }
}
