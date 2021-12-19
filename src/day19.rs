use super::utils::read_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type RelPos = [i32; 3];

fn rotated_positions(initial: &RelPos) -> Vec<RelPos> {
    let mut rotations: Vec<RelPos> = Vec::new();

    for indices in (0..3).permutations(3) {
        let mut n_swaps = 2;
        for i in 0..3 {
            if indices[i] == i && n_swaps > 0 {
                n_swaps -= 1;
            }
        }
        let n_minuses: [usize; 2] = if n_swaps % 2 == 0 { [0, 2] } else { [1, 3] };

        for n_minus in n_minuses {
            for minus_indices in (0..3).combinations(n_minus) {
                let mut rotated: RelPos = [0, 0, 0];
                for i in 0..3 {
                    rotated[i] =
                        initial[indices[i]] * if minus_indices.contains(&i) { -1 } else { 1 };
                }
                rotations.push(rotated);
            }
        }
    }
    rotations
}

#[derive(Debug, Clone)]
struct ScannerData {
    beacons: HashSet<RelPos>,
    len_to_closest_pos: HashMap<RelPos, Option<f32>>,
}

impl ScannerData {
    fn new(beacons: HashSet<RelPos>) -> ScannerData {
        let mut ltcps: HashMap<RelPos, Option<f32>> = HashMap::new();
        for b in &beacons {
            let mut ltcp: Option<f32> = None;
            for b2 in &beacons {
                if b2[0] > b[0] && b2[1] > b[1] && b2[2] > b[2] {
                    let len = ((2 as f32).powf((b2[0] - b[0]) as f32)
                        + (2 as f32).powf((b2[1] - b[1]) as f32)
                        + (2 as f32).powf((b2[2] - b[2]) as f32))
                    .sqrt();
                    if let Some(cur_len) = ltcp {
                        if cur_len > len {
                            ltcp = Some(len);
                        }
                    } else {
                        ltcp = Some(len);
                    }
                }
            }
            ltcps.insert(b.clone(), ltcp);
        }
        ScannerData {
            beacons,
            len_to_closest_pos: ltcps,
        }
    }

    fn print(&self) {
        println!("\nscanner data:");
        for b in &self.beacons {
            for coord in b {
                print!("{:#5} ", coord)
            }
            println!();
        }
    }

    fn parse(s: &String) -> Vec<ScannerData> {
        let mut res: Vec<ScannerData> = Vec::new();
        let mut beacons: HashSet<RelPos> = HashSet::new();
        for line in s.lines() {
            if !line.contains(",") {
                if beacons.len() > 0 {
                    res.push(ScannerData::new(beacons));
                    beacons = HashSet::new();
                }
            } else {
                let pos: Vec<i32> = line.split(',').map(|ss| ss.parse().unwrap()).collect();
                beacons.insert([pos[0], pos[1], pos[2]]);
            }
        }
        if beacons.len() > 0 {
            res.push(ScannerData::new(beacons));
        }
        res
    }

    fn rotated(&self) -> Vec<ScannerData> {
        let mut beacons_rotated: Vec<HashSet<RelPos>> = Vec::new();
        for _ in 0..24 {
            beacons_rotated.push(HashSet::new());
        }
        for b in &self.beacons {
            for (i, rb) in rotated_positions(b).into_iter().enumerate() {
                beacons_rotated[i].insert(rb);
            }
        }
        beacons_rotated
            .into_iter()
            .map(|b| ScannerData::new(b))
            .collect()
    }

    fn match_and_merge_with_rotations(&self, other: &ScannerData) -> Option<(ScannerData, RelPos)> {
        for rotated_other in other.rotated() {
            return match self.match_and_merge(&rotated_other) {
                Some(res) => Some(res),
                None => {
                    continue;
                }
            };
        }
        None
    }

    fn match_and_merge(&self, other: &ScannerData) -> Option<(ScannerData, RelPos)> {
        for b1 in &self.beacons {
            let ltcp1 = self.len_to_closest_pos.get(b1).unwrap();
            if let None = ltcp1 {
                continue;
            }
            let ltcp1 = ltcp1.unwrap();
            for b2 in &other.beacons {
                // println!("{:?}", b2);
                // println!("{:?}", other.len_to_closest_pos);

                let ltcp2 = other.len_to_closest_pos.get(b2).unwrap();
                if let None = ltcp2 {
                    continue;
                }
                let ltcp2 = ltcp2.unwrap();
                if (ltcp1 - ltcp2).abs() > 0.1 {
                    continue;
                }

                // aligning b2 with b1
                let offset: RelPos = [b1[0] - b2[0], b1[1] - b2[1], b1[2] - b2[2]];
                let offsetted_other_beacons: HashSet<RelPos> = (&other.beacons)
                    .iter()
                    .map(|pos| [pos[0] + offset[0], pos[1] + offset[1], pos[2] + offset[2]])
                    .collect();
                let matching_n = offsetted_other_beacons.intersection(&self.beacons).count();
                if matching_n >= 12 {
                    return Some((
                        ScannerData::new(
                            offsetted_other_beacons
                                .union(&self.beacons)
                                .map(|&v| v)
                                .collect(),
                        ),
                        offset,
                    ));
                }
            }
        }
        None
    }
}

fn manhattan_distance(p1: &RelPos, p2: &RelPos) -> i32 {
    (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs() + (p1[2] - p2[2]).abs()
}

pub fn beacons() {
    let input = read_input(19, false);

    let scanners = ScannerData::parse(&input);

    let mut total: ScannerData = scanners[0].clone();
    let mut merged_indices: Vec<usize> = vec![0];
    let mut scanner_offsets: Vec<RelPos> = Vec::new();

    loop {
        for (i, s) in scanners.iter().enumerate() {
            if !merged_indices.contains(&i) {
                if let Some((new_total, scanner_offset)) = total.match_and_merge_with_rotations(s) {
                    scanner_offsets.push(scanner_offset);
                    new_total.print();
                    total = new_total;
                    merged_indices.push(i);
                    break;
                }
            }
        }
        if merged_indices.len() == scanners.len() {
            break;
        }
    }

    println!("{}", total.beacons.len());
    println!("{:?}", scanner_offsets);
    println!(
        "{}",
        &scanner_offsets
            .iter()
            .cartesian_product(&scanner_offsets)
            .map(|(o1, o2)| manhattan_distance(o1, o2))
            .max()
            .unwrap()
    )
}
