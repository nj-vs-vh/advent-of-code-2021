use super::utils::read_input;
use std::clone::Clone;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Eq, Hash, PartialEq, Clone, Debug, Ord, PartialOrd)]
enum CaveName {
    Start,
    Small(String),
    Big(String),
    End,
}

impl fmt::Display for CaveName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            CaveName::Start => "start",
            CaveName::End => "end",
            CaveName::Small(string) => string,
            CaveName::Big(string) => string,
        };
        write!(f, "{}", out)
    }
}

impl CaveName {
    fn parse(s: &str) -> CaveName {
        let res = match s {
            "start" => CaveName::Start,
            "end" => CaveName::End,
            other => {
                let s = String::from(other);
                // checking only first letter
                if s.chars().next().unwrap() < 'Z' {
                    CaveName::Big(s)
                } else {
                    CaveName::Small(s)
                }
            }
        };
        res
    }

    fn is_big(&self) -> bool {
        match self {
            CaveName::Big(_) => true,
            _ => false,
        }
    }

    fn is_small(&self) -> bool {
        match self {
            CaveName::Small(_) => true,
            _ => false,
        }
    }

    fn cmp_big_small(&self, other: &CaveName) -> std::cmp::Ordering {
        fn cmp_with_other(name: &String, other: &CaveName) -> std::cmp::Ordering {
            match other {
                CaveName::Big(other_name) => name.cmp(other_name),
                CaveName::Small(other_name) => name.cmp(other_name),
                _ => std::cmp::Ordering::Less,
            }
        }
        match self {
            CaveName::Big(name) => cmp_with_other(name, other),
            CaveName::Small(name) => cmp_with_other(name, other),
            _ => std::cmp::Ordering::Greater,
        }
    }
}

type Network = HashMap<CaveName, Vec<CaveName>>;

fn parse_cave_network(input: String) -> Network {
    let mut network: Network = HashMap::new();
    for line in input.lines() {
        let cave_names: Vec<CaveName> = line.split('-').map(CaveName::parse).collect();
        for cave_name in &cave_names {
            if !network.contains_key(cave_name) {
                network.insert(cave_name.clone(), Vec::new());
            }
        }
        for (i_from, i_to) in [(1, 0), (0, 1)] {
            network
                .get_mut(&cave_names[i_from])
                .unwrap()
                .push(cave_names[i_to].clone());
        }
    }
    for conn_to in network.values_mut() {
        conn_to.sort_by(|a, b| a.cmp_big_small(b));
    }
    network
}

#[allow(dead_code)]
fn print_cave_network(network: &Network) {
    for (name, connected_to) in network {
        print!("{}: ", name);
        for name in connected_to {
            print!("{}, ", name)
        }
        println!();
    }
}

fn find_paths<'a>(network: &Network, part2: bool) -> Vec<Vec<&CaveName>> {
    fn find_paths_recursive<'a>(
        network: &'a Network,
        now_at: &'a CaveName,
        visited: &HashSet<&CaveName>,
        one_small_cave_visited_twice: bool,
    ) -> Vec<Vec<&'a CaveName>> {
        if *now_at == CaveName::End {
            return vec![vec![now_at; 1]; 1];
        }
        let mut visited = visited.clone();
        visited.insert(now_at);

        let mut res: Vec<Vec<&'a CaveName>> = Vec::new();
        for next in network.get(now_at).unwrap() {
            let mut visiting_small_cave_second_time = false;
            let visit_next = match visited.contains(next) {
                false => true,
                true => {
                    if next.is_big() {
                        true
                    } else if next.is_small() {
                        if one_small_cave_visited_twice {
                            false
                        } else {
                            visiting_small_cave_second_time = true;
                            true
                        }
                    } else {
                        false
                    }
                }
            };
            if visit_next {
                for path in find_paths_recursive(
                    network,
                    next,
                    &visited,
                    one_small_cave_visited_twice || visiting_small_cave_second_time,
                )
                .iter_mut()
                {
                    let mut full_path = vec![now_at; 1];
                    full_path.append(path);
                    res.push(full_path);
                }
            }
        }
        res
    }

    find_paths_recursive(network, &CaveName::Start, &HashSet::new(), !part2)
}

pub fn path_in_caves() {
    let input = read_input(12, false);

    let network = parse_cave_network(input);
    // print_cave_network(&network);

    let paths = find_paths(&network, false);
    println!("Total paths to the end: {}", paths.len());

    let paths = find_paths(&network, true);
    // for path in &paths {
    //     println!(
    //         "{}",
    //         path.iter()
    //             .map(|cn| format!("{}", cn))
    //             .collect::<Vec<String>>()
    //             .join(",")
    //     );
    // }
    println!(
        "Total paths to the end with one small cave visited twice: {}",
        paths.len()
    );
}
