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

    fn result(&self, n1: &i32, n2: &i32) -> i32 {
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
            Op::Inp => {
                panic!("'result' method is not defined for 'inp' operation")
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

type Value2Digits = HashMap<i32, HashSet<u8>>; // {numeric value: {set of digits resulting in this input}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Variable(Var),
    Number(i32),
    InputMapping(usize, Value2Digits), // idx in input: Value2Digits @ this idx}
    Expression(Op, Vec<Value>),
}

impl Value {
    fn print(&self) {
        fn print_recursive(v: &Value, indent: usize) {
            let indentation: String = iter::repeat(' ').take(indent).collect();
            match v {
                Value::Number(n) => println!("{}{}", indentation, n),
                Value::Variable(v) => println!("{}{:?}", indentation, v),
                Value::Expression(op, operands) => {
                    println!("{}{:?}", indentation, op);
                    for v in operands {
                        print_recursive(v, indent + 4);
                    }
                }
                Value::InputMapping(inp_idx, im) => {
                    println!("{}(inp[{}]) {:?}", indentation, inp_idx, im)
                }
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
        let mut tirivial_map: Value2Digits = HashMap::new();
        for digit in 0..10 {
            let mut trivial_set = HashSet::new();
            trivial_set.insert(digit);
            tirivial_map.insert(digit as i32, trivial_set);
        }
        return (op, value1, Value::InputMapping(digit_idx, tirivial_map));
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
                return Value::Number(op.result(n1, n2));
            } else {
                // if one operand is an input mapping and the other is a number -- apply operation to all mapping values
                fn update_mapping_with_number(
                    op: &Op,
                    v_map: &Value,
                    v_num: &Value,
                    passed_in_order: bool,
                ) -> Option<Value> {
                    if let (Value::Number(n), Value::InputMapping(inp_dig, v2g)) = (v_num, v_map) {
                        let mut new_v2g: Value2Digits = HashMap::new();
                        for (value, digit_set) in v2g {
                            let new_value = if passed_in_order {
                                op.result(value, n)
                            } else {
                                op.result(n, value)
                            };
                            if !new_v2g.contains_key(&new_value) {
                                new_v2g.insert(new_value, HashSet::new());
                            }
                            new_v2g
                                .get_mut(&new_value)
                                .unwrap()
                                .extend(digit_set.into_iter());
                            // new_v2g.insert(new_value, digit_set.clone());
                        }
                        return Some(Value::InputMapping(*inp_dig, new_v2g));
                    }
                    None
                }

                if let Some(v) = update_mapping_with_number(op, v1, v2, true) {
                    return v;
                }
                if let Some(v) = update_mapping_with_number(op, v2, v1, false) {
                    return v;
                }

                // if both are mappings for the same digit index -- merge them (cartesian product, intersecting digit value sets)
                if let (
                    Value::InputMapping(digit_idx_1, map_1),
                    Value::InputMapping(digit_idx_2, map_2),
                ) = (v1, v2)
                {
                    if digit_idx_1 == digit_idx_2 {
                        let mut new_map: Value2Digits = HashMap::new();
                        for (value_1, digit_values_1) in map_1 {
                            for (value_2, digit_values_2) in map_2 {
                                let intersection: HashSet<u8> = digit_values_1
                                    .iter()
                                    .filter(|&dv1| digit_values_2.contains(dv1))
                                    .cloned()
                                    .collect();
                                if intersection.len() > 0 {
                                    new_map.insert(op.result(value_1, value_2), intersection);
                                }
                            }
                        }
                        return Value::InputMapping(*digit_idx_1, new_map);
                    }
                }

                // otherwise -- return Expression with two of them
                return Value::Expression(*op, vec![v1.clone(), v2.clone()]);
            }
        }
    }
}

const N_INPUTS: usize = 1;

pub fn arithmetic_logic_unit() {
    let input = read_input(24, false);

    let mut var_values: VariableValues = HashMap::new();
    var_values.insert(Var::X, Value::Number(0));
    var_values.insert(Var::Y, Value::Number(0));
    var_values.insert(Var::Z, Value::Number(0));
    var_values.insert(Var::W, Value::Number(0));

    // let input_digits: [i32; N_INPUTS] = [1];
    let mut next_digit: usize = 0;
    for (i, line) in input.lines().enumerate() {
        println!("\n{}. {}", i, line);
        if i > 100 {
            break;
        }
        let (op, v1, v2) = parse_expression(line, next_digit);
        if op == Op::Inp {
            next_digit += 1;
        }
        let assign_to = &v1;
        if let Value::Variable(assign_to) = assign_to {
            let result = perform_operation(&op, &v1, &v2, &var_values);
            var_values.insert(*assign_to, result);
        } else {
            panic!("Expecting the first operand to be a variable to assign result to")
        }
    }

    for (var, value) in &var_values {
        print!("{:?} = ", var);
        value.print();
    }
}
