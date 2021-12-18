use super::utils::read_input;
use std::iter;

#[derive(Debug, Clone)]
enum NodeParentRef {
    Left(usize),
    Right(usize),
}

#[derive(Debug, Clone)]
enum NodeContent {
    Number(u32),
    Bracket(usize, usize), // indices
}

#[derive(Debug, Clone)]
struct Node {
    parent_ref: Option<NodeParentRef>,
    content: NodeContent,
}

#[derive(Debug, Clone)]
struct SnailfishNumber {
    root_idx: usize,
    nodes: Vec<Node>,
}

const DEBUG: bool = false;

impl SnailfishNumber {
    fn print(&self) {
        fn print_recursive(nodes: &Vec<Node>, idx: usize, indent: usize) {
            let indentation: String = iter::repeat(' ').take(indent).collect();
            let node = &nodes[idx];
            match node.content {
                NodeContent::Number(num) => {
                    println!("{}({}) {}", indentation, idx, num);
                }
                NodeContent::Bracket(left_idx, right_idx) => {
                    print_recursive(nodes, right_idx, indent + 5);
                    println!("{}({}) <", indentation, idx);
                    print_recursive(nodes, left_idx, indent + 5);
                }
            }
        }

        println!("\n\n");
        print_recursive(&self.nodes, self.root_idx, 0)
    }

    fn sum(&self, other: &SnailfishNumber) -> SnailfishNumber {
        let mut self_ = self.clone();
        let mut other = other.clone();
        self_.shift_indices(1, true, None);
        other.shift_indices(self_.nodes.len() + 1, true, None);
        self_.nodes[self_.root_idx].parent_ref = Some(NodeParentRef::Left(0));
        other.nodes[other.root_idx].parent_ref = Some(NodeParentRef::Right(0));

        let new_root = Node {
            parent_ref: None,
            content: NodeContent::Bracket(
                self_.root_idx + 1,
                other.root_idx + 1 + self_.nodes.len(),
            ),
        };

        let mut all_nodes: Vec<Node> = Vec::new();
        all_nodes.push(new_root);
        all_nodes.extend(self_.nodes);
        all_nodes.extend(other.nodes);

        SnailfishNumber {
            root_idx: 0,
            nodes: all_nodes,
        }
    }

    fn shift_indices(&mut self, amt: usize, right: bool, starting_from: Option<usize>) {
        let starting_from = starting_from.unwrap_or(0);
        let shifted = |idx: &usize| {
            if *idx >= starting_from {
                if right {
                    idx + amt
                } else {
                    idx - amt
                }
            } else {
                *idx
            }
        };
        if !right {
            assert!(amt <= starting_from);
        }
        for node in self.nodes.iter_mut() {
            if let Some(pref) = &node.parent_ref {
                node.parent_ref = match pref {
                    NodeParentRef::Left(idx) => Some(NodeParentRef::Left(shifted(idx))),
                    NodeParentRef::Right(idx) => Some(NodeParentRef::Right(shifted(idx))),
                }
            }
            if let NodeContent::Bracket(left_idx, right_idx) = &node.content {
                node.content = NodeContent::Bracket(shifted(left_idx), shifted(right_idx));
            }
        }
    }

    fn parse(s: &str) -> SnailfishNumber {
        fn _parse_snailfish_num_recursive(s: &str) -> (SnailfishNumber, &str) {
            let first_ch = s.chars().next().unwrap();
            if DEBUG {
                println!("'{}', first char is {}", s, first_ch);
            }
            match first_ch {
                '[' => {
                    if DEBUG {
                        println!("parsing as bracket");
                    }
                    let (left_num, rest) = _parse_snailfish_num_recursive(&s[1..]);
                    let (right_num, rest) = _parse_snailfish_num_recursive(&rest[1..]);
                    (left_num.sum(&right_num), &rest[1..])
                }
                _ => {
                    if DEBUG {
                        println!("parsing as number");
                    }
                    let mut first_non_digit_idx: usize = 0;
                    for (i, ch) in s.chars().enumerate() {
                        if !ch.is_digit(10) {
                            first_non_digit_idx = i;
                            break;
                        }
                    }
                    if first_non_digit_idx == 0 {
                        panic!("Something's wrong when parsing numeric value")
                    }
                    let num: u32 = s[..first_non_digit_idx]
                        .parse()
                        .expect("Can't parse numeric value in snailfish number");
                    let root_node = Node {
                        parent_ref: None,
                        content: NodeContent::Number(num),
                    };
                    (
                        SnailfishNumber {
                            root_idx: 0,
                            nodes: Vec::from([root_node]),
                        },
                        &s[first_non_digit_idx..],
                    )
                }
            }
        }

        _parse_snailfish_num_recursive(s).0
    }

    fn explode(&mut self) -> bool {
        fn find_exploding_node_recursive(
            nodes: &Vec<Node>,
            idx: usize,
            depth_current: usize,
        ) -> Option<usize> {
            let node = &nodes[idx];
            if let NodeContent::Bracket(left_idx, right_idx) = node.content {
                if let NodeContent::Number(_) = &nodes[left_idx].content {
                    if let NodeContent::Number(_) = &nodes[right_idx].content {
                        return if depth_current >= 4 { Some(idx) } else { None };
                    }
                }
                return match find_exploding_node_recursive(nodes, left_idx, depth_current + 1) {
                    None => find_exploding_node_recursive(nodes, right_idx, depth_current + 1),
                    Some(idx) => Some(idx),
                };
            } else {
                return None;
            }
        }

        let exploding_node_idx = find_exploding_node_recursive(&self.nodes, self.root_idx, 0);

        if let None = exploding_node_idx {
            return false;
        }

        fn find_exploding_node_left_target(nodes: &Vec<Node>, idx: usize) -> Option<(usize, u32)> {
            if let NodeContent::Bracket(expleft_idx, _) = &nodes[idx].content {
                if let NodeContent::Number(left_num) = &nodes[*expleft_idx].content {
                    let mut expleft_idx = *expleft_idx;
                    while expleft_idx > 0 {
                        expleft_idx -= 1;
                        if let NodeContent::Number(_) = &nodes[expleft_idx].content {
                            return Some((expleft_idx, *left_num));
                        }
                    }
                    return None;
                }
            }
            panic!("Error in find_exploding_node_recursive");
        }
        fn find_exploding_node_right_target(nodes: &Vec<Node>, idx: usize) -> Option<(usize, u32)> {
            if let NodeContent::Bracket(_, expright_idx) = &nodes[idx].content {
                if let NodeContent::Number(right_num) = &nodes[*expright_idx].content {
                    let mut expright_idx = *expright_idx;
                    while expright_idx < nodes.len() - 1 {
                        expright_idx += 1;
                        if let NodeContent::Number(_) = &nodes[expright_idx].content {
                            return Some((expright_idx, *right_num));
                        }
                    }
                    return None;
                }
            }
            panic!("Error in find_exploding_node_recursive");
        }

        let exploding_node_idx = exploding_node_idx.unwrap();

        for func in [
            find_exploding_node_right_target,
            find_exploding_node_left_target,
        ] {
            if let Some((target_idx, add)) = func(&self.nodes, exploding_node_idx) {
                if let NodeContent::Number(num) = self.nodes[target_idx].content {
                    self.nodes[target_idx].content = NodeContent::Number(num + add)
                }
            }
        }
        self.nodes[exploding_node_idx].content = NodeContent::Number(0);
        self.nodes.remove(exploding_node_idx + 1);
        self.nodes.remove(exploding_node_idx + 1);
        self.shift_indices(2, false, Some(exploding_node_idx + 1));
        true
    }

    fn split(&mut self) -> bool {
        let mut split_idx: Option<usize> = None;
        for (idx, node) in self.nodes.iter().enumerate() {
            if let NodeContent::Number(num) = node.content {
                if num >= 10 {
                    split_idx = Some(idx);
                    break;
                }
            }
        }
        if let None = split_idx {
            return false;
        }
        let split_idx = split_idx.unwrap();

        fn get_nodes_to_insert(nodes: &Vec<Node>, idx: usize) -> (Node, Node) {
            if let NodeContent::Number(num2split) = &nodes[idx].content {
                let left_num = num2split / 2;
                let right_num = num2split - left_num;
                return (
                    Node {
                        content: NodeContent::Number(left_num),
                        parent_ref: Some(NodeParentRef::Left(idx)),
                    },
                    Node {
                        content: NodeContent::Number(right_num),
                        parent_ref: Some(NodeParentRef::Right(idx)),
                    },
                );
            }
            panic!();
        }

        let (right_node, left_node) = get_nodes_to_insert(&self.nodes, split_idx);
        self.nodes.insert(split_idx + 1, left_node);
        self.nodes.insert(split_idx + 1, right_node);
        self.shift_indices(2, true, Some(split_idx + 1));
        self.nodes[split_idx].content = NodeContent::Bracket(split_idx + 1, split_idx + 2);
        true
    }

    fn reduce(&mut self) -> bool {
        self.explode() || self.split()
    }

    fn magnitude(&self) -> u32 {
        fn node_magnitude(nodes: &Vec<Node>, idx: usize) -> u32 {
            let node = &nodes[idx];
            match node.content {
                NodeContent::Number(num) => num,
                NodeContent::Bracket(left_idx, right_idx) => {
                    3 * node_magnitude(nodes, left_idx) + 2 * node_magnitude(nodes, right_idx)
                }
            }
        }

        node_magnitude(&self.nodes, self.root_idx)
    }
}

pub fn snailfish_math() {
    let input = read_input(18, false);

    let numbers: Vec<SnailfishNumber> = input.lines().map(SnailfishNumber::parse).collect();

    let mut res = numbers[0].clone();
    for sn in &numbers {
        res = res.sum(sn);
        // res.print();
        while res.reduce() {
            // res.print();
        }
    }

    res.print();

    println!("magnitude is {}", res.magnitude());

    let mut summ: SnailfishNumber;
    let mut max_magnitude: u32 = 0;
    for first_idx in 0..numbers.len() {
        for second_idx in 0..numbers.len() {
            if first_idx == second_idx {
                continue;
            }
            summ = numbers[first_idx].sum(&numbers[second_idx]);
            while summ.reduce() {
                // res.print();
            }
            let magn = summ.magnitude();
            if magn > max_magnitude {
                max_magnitude = magn;
            }
        }
    }

    println!("max pairwise magnitude: {}", max_magnitude);
}
