use super::utils::read_input;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Node {
    i: usize,
    j: usize,
}

struct Map {
    risks: Vec<Vec<u8>>,
    imax: usize,
    jmax: usize,
}

impl Map {
    fn parse(s: &String) -> Map {
        let mut risks: Vec<Vec<u8>> = Vec::new();
        let mut row: Vec<u8>;
        for line in s.lines() {
            row = Vec::new();
            for ch in line.chars() {
                row.push(ch.to_digit(10).unwrap() as u8);
            }
            risks.push(row);
        }
        let imax = risks.len() - 1;
        let jmax = risks.iter().next().unwrap().len() - 1;
        Map { risks, imax, jmax }
    }

    fn print(&self) {
        for i in 0..=self.imax {
            for j in 0..=self.jmax {
                print!("{}", self.risks[i][j])
            }
            println!();
        }
    }

    fn tiled_map(&self) -> Map {
        let imax_tiled = (self.imax + 1) * 5 - 1;
        let jmax_tiled = (self.jmax + 1) * 5 - 1;
        let mut risks_new: Vec<Vec<u8>> = vec![vec![0; jmax_tiled + 1]; imax_tiled + 1];
        for i in 0..=imax_tiled {
            for j in 0..=jmax_tiled {
                let i_self = i % (self.imax + 1);
                let j_self = j % (self.jmax + 1);
                let i_tile = i / (self.imax + 1);
                let j_tile = j / (self.jmax + 1);
                risks_new[i][j] =
                    (self.risks[i_self][j_self] + i_tile as u8 + j_tile as u8 - 1) % 9 + 1;
            }
        }
        Map {
            risks: risks_new,
            imax: imax_tiled,
            jmax: jmax_tiled,
        }
    }
}

impl Node {
    fn neighbors(&self, map: &Map) -> Vec<(Node, u8)> {
        let mut res: Vec<(Node, u8)> = Vec::new();
        let mut i2: i16;
        let mut j2: i16;
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            i2 = self.i as i16 - di;
            j2 = self.j as i16 - dj;
            if i2 < 0 || i2 > map.imax as i16 || j2 < 0 || j2 > map.jmax as i16 {
                continue;
            }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            res.push((Node { i: i2, j: j2 }, map.risks[i2][j2]))
        }
        res
    }

    fn print(&self) {
        println!("{}, {}", self.i, self.j);
    }
}

#[derive(Clone, Debug)]
struct PriorityQueueRecord {
    node: Node,
    cumcost: u32,
    from: Option<Node>,
}

fn dijkstra(map: &Map) -> u32 {
    let start = PriorityQueueRecord {
        node: Node { i: 0, j: 0 },
        cumcost: 0,
        from: None,
    };
    let end_node = Node {
        i: map.imax,
        j: map.jmax,
    };

    let mut now = &start;

    let mut discarded: HashSet<Node> = HashSet::new();

    // too lazy to implement an actual queue here...
    let mut priority_queue: HashMap<Node, PriorityQueueRecord> = HashMap::new();
    let mut new_priority_queue: HashMap<Node, PriorityQueueRecord> = HashMap::new();

    loop {
        // now.node.print();
        for (next_node, move_cost) in now.node.neighbors(map) {
            if next_node == end_node {
                return now.cumcost + move_cost as u32;
            }
            if discarded.contains(&next_node) {
                continue;
            }
            if let Some(from_node) = &now.from {
                if next_node == *from_node {
                    continue;
                }
            }
            let new_cumcost: u32 = now.cumcost + move_cost as u32;
            if let Some(current_node_record) = priority_queue.get(&next_node) {
                if current_node_record.cumcost < new_cumcost {
                    continue;
                }
            }
            new_priority_queue.insert(
                next_node.clone(),
                PriorityQueueRecord {
                    node: next_node,
                    cumcost: new_cumcost,
                    from: Some(now.node.clone()),
                },
            );
        }
        discarded.insert(now.node);
        new_priority_queue.remove(&now.node);
        priority_queue = new_priority_queue.clone();
        now = priority_queue.values().min_by_key(|r| r.cumcost).unwrap();
    }
}

pub fn chitons() {
    let input = read_input(15, false);

    let map = Map::parse(&input);
    println!("cost is {}", dijkstra(&map));

    let large_map = map.tiled_map();
    println!("cost (on large map) is {}", dijkstra(&large_map));
}
