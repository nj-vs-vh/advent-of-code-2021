use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

const ROOM_LEN: usize = 4;
const HALLWAY_LEN: usize = 7; // excluding 4 cells above the rooms
const ROOM_N: usize = 4;

const DEBUG: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn parse(ch: char) -> Amphipod {
        match ch {
            'A' => Amphipod::A,
            'B' => Amphipod::B,
            'C' => Amphipod::C,
            'D' => Amphipod::D,
            _ => panic!("Can't parse amphipod from {}", ch),
        }
    }
    fn step_cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
    fn target_room(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }
    fn aschar(&self) -> char {
        match self {
            Amphipod::A => 'A',
            Amphipod::B => 'B',
            Amphipod::C => 'C',
            Amphipod::D => 'D',
        }
    }
}

type Room = [Option<Amphipod>; ROOM_LEN];

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    rooms: [Room; ROOM_N],
    hallway: [Option<Amphipod>; HALLWAY_LEN],
}

fn hallway_idx_to_x(i: usize) -> usize {
    match i {
        0..=1 => i,
        2 => 3,
        3 => 5,
        4 => 7,
        5..=6 => i + 4,
        _ => {
            panic!();
        }
    }
}

fn room_idx_to_x(i: usize) -> usize {
    2 + i * 2
}

impl State {
    fn display(&self) {
        let mut hallway_full: Vec<Option<Amphipod>> = vec![None; HALLWAY_LEN + ROOM_N];
        for hw in 0..HALLWAY_LEN {
            if let Some(a) = self.hallway[hw] {
                hallway_full[hallway_idx_to_x(hw)] = Some(a);
            }
        }
        for oa in hallway_full {
            match oa {
                None => print!("."),
                Some(a) => print!("{}", a.aschar()),
            }
        }
        println!();
        for room_depth in 0..ROOM_LEN {
            let mut rooms_x: Vec<Option<Amphipod>> = vec![None; HALLWAY_LEN + ROOM_N];
            for r_i in 0..ROOM_N {
                if let Some(a) = self.rooms[r_i][room_depth] {
                    rooms_x[room_idx_to_x(r_i)] = Some(a);
                }
            }
            for oa in rooms_x {
                match oa {
                    None => print!(" "),
                    Some(a) => print!("{}", a.aschar()),
                }
            }
            println!();
        }
    }

    fn parse_init(s: &str) -> State {
        let mut rooms: [Room; ROOM_N] = Default::default();
        for (i_room, room_s) in s.split(' ').enumerate() {
            for (i_ch, ch) in room_s.chars().enumerate() {
                rooms[i_room][i_ch] = Some(Amphipod::parse(ch));
            }
        }
        State {
            rooms,
            hallway: Default::default(),
        }
    }

    fn is_end(&self) -> bool {
        fn room_eq(r1: &Room, r2: &Room) -> bool {
            r1.iter().zip(r2.iter()).all(|(a, b)| a == b)
        }

        room_eq(&self.rooms[0], &[Some(Amphipod::A); ROOM_LEN])
            && room_eq(&self.rooms[1], &[Some(Amphipod::B); ROOM_LEN])
            && room_eq(&self.rooms[2], &[Some(Amphipod::C); ROOM_LEN])
            && room_eq(&self.rooms[3], &[Some(Amphipod::D); ROOM_LEN])
    }

    fn next_states(&self) -> Vec<(usize, State)> {
        let mut res: Vec<(usize, State)> = Vec::new();

        // next states resulting from moving amphipod from hallway to the room
        for hw in 0..HALLWAY_LEN {
            if let Some(amph_to_move) = self.hallway[hw] {
                let target_room = amph_to_move.target_room();

                if DEBUG {
                    println!(
                        "{}: hallway {} -> room {}",
                        amph_to_move.aschar(),
                        hw,
                        target_room
                    );
                }

                // checking if amphipod can enter their room
                let mut amphs_in_target: usize = 0;
                let mut matching_amphs_in_target: usize = 0;
                for i in 0..ROOM_LEN {
                    if let Some(amph) = self.rooms[target_room][i] {
                        amphs_in_target += 1;
                        if amph == amph_to_move {
                            matching_amphs_in_target += 1;
                        }
                    }
                }
                if DEBUG {
                    println!(
                        "\tin room {}: {} total amphipods, {} of them match",
                        target_room, amphs_in_target, matching_amphs_in_target
                    );
                }
                if amphs_in_target > matching_amphs_in_target {
                    // not moving there, rule 2
                    continue;
                }
                let target_depth_in_room = ROOM_LEN - 1 - matching_amphs_in_target;
                // checking if amphipod can even reach the entrance to their room (not blocked by some other amphipod)
                let hw_next_to_room = target_room + 2;
                let between_range = if hw < hw_next_to_room {
                    (hw + 1)..hw_next_to_room
                } else {
                    hw_next_to_room..hw
                };
                if !self.hallway[between_range].iter().all(|hw| hw == &None) {
                    if DEBUG {
                        println!("\tmove is blocked!")
                    }
                    // not moving the path is blocked
                    continue;
                }
                let mut new_state = self.clone();
                new_state.hallway[hw] = None;
                new_state.rooms[target_room][target_depth_in_room] = Some(amph_to_move);
                // move cost
                let from_x = hallway_idx_to_x(hw);
                let to_x = room_idx_to_x(target_room);
                let horizontal_move_cost = if to_x > from_x {
                    to_x - from_x
                } else {
                    from_x - to_x
                };
                let move_cost =
                    (horizontal_move_cost + target_depth_in_room + 1) * amph_to_move.step_cost();

                res.push((move_cost, new_state));
            }
        }

        // next states resulting from moving amphipods from rooms to hallway
        for i_room in 0..ROOM_N {
            // checking that the room isn't settled yet
            if self.rooms[i_room].iter().all(|oa| {
                if let Some(a) = oa {
                    a.target_room() == i_room
                } else {
                    false
                }
            }) {
                if DEBUG {
                    println!("room {} if already OK", i_room);
                }
                continue;
            }
            let come_out = self.rooms[i_room]
                .iter()
                .enumerate()
                .filter(|&(_, oa)| oa != &None)
                .next();
            if come_out == None {
                continue;
            }
            let (coming_out_depth, coming_out_amph) = come_out.unwrap();
            // looking for placed where it could go
            let mut target_hws: Vec<usize> = Vec::new();
            let mut target_hw = i_room + 2; // right from room
            while target_hw < HALLWAY_LEN && self.hallway[target_hw] == None {
                target_hws.push(target_hw);
                target_hw += 1;
            }
            let mut target_hw = i_room + 1; // left from room
            while self.hallway[target_hw] == None {
                target_hws.push(target_hw);
                if target_hw == 0 {
                    break;
                }
                target_hw -= 1;
            }

            if DEBUG {
                println!(
                    "{} from room {} comes to hallway: {:?}",
                    coming_out_amph.unwrap().aschar(),
                    i_room,
                    target_hws,
                );
            }

            for target_hw in target_hws {
                let mut new_state = self.clone();
                new_state.rooms[i_room][coming_out_depth] = None;
                new_state.hallway[target_hw] = *coming_out_amph;

                // move cost
                let from_x = room_idx_to_x(i_room);
                let to_x = hallway_idx_to_x(target_hw);
                let horizontal_move_cost = if to_x > from_x {
                    to_x - from_x
                } else {
                    from_x - to_x
                };
                let move_cost = (horizontal_move_cost + coming_out_depth + 1)
                    * coming_out_amph.unwrap().step_cost();

                res.push((move_cost, new_state));
            }
        }

        res
    }
}

const DISPLAY: bool = false;

fn dijkstra(state: State) -> i32 {
    let mut pq: PriorityQueue<State, i32> = PriorityQueue::new();
    let mut visited: HashSet<State> = HashSet::new();
    let mut previous: HashMap<State, State> = HashMap::new();
    let mut now_at: State = state;
    let mut neg_total_cost: i32 = 0;

    loop {
        for (move_cost, new_state) in now_at.next_states() {
            if visited.contains(&new_state) {
                continue;
            }
            previous.insert(new_state.clone(), now_at.clone());
            let priority = neg_total_cost - (move_cost as i32);
            if let Some(existing_priority) = pq.get_priority(&new_state) {
                if *existing_priority > priority {
                    continue;
                }
            }
            pq.push(new_state, priority);
        }
        visited.insert(now_at.clone());
        let next_with_cost = pq.pop().unwrap();
        now_at = next_with_cost.0;
        if now_at.is_end() {
            break;
        }
        neg_total_cost = next_with_cost.1;
        if DISPLAY {
            println!("\n{}", -neg_total_cost);
            now_at.display();
        }
    }

    loop {
        println!();
        now_at.display();
        if let Some(state) = previous.get(&now_at) {
            now_at = state.clone();
        } else {
            break;
        }
    }

    return -neg_total_cost;
}

pub fn amphipod_rooms() {
    let init = "DDDC ACBA CBAB DACB"; // left to right, top to bottom in each room
    let state = State::parse_init(init);
    println!("the most efficient path takes {}", dijkstra(state));
}
