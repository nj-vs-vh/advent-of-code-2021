use super::utils::read_input;
use std::collections::HashSet;

#[derive(Debug)]
struct Heightmap {
    map: Vec<Vec<u8>>,
    delta_up: Vec<Vec<i16>>,
    delta_right: Vec<Vec<i16>>,
    delta_down: Vec<Vec<i16>>,
    delta_left: Vec<Vec<i16>>,
}

fn vec2d_size(vv: &Vec<Vec<u8>>) -> (usize, usize) {
    (vv.iter().next().unwrap().len(), vv.len())
}

impl Heightmap {
    const MAX: u8 = 9;

    fn parse(s: &String) -> Heightmap {
        let map: Vec<Vec<u8>> = s
            .lines()
            .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect())
            .collect();

        fn delta_map(map: &Vec<Vec<u8>>, di: i16, dj: i16) -> Vec<Vec<i16>> {
            let mut delta_map: Vec<Vec<i16>> = map
                .iter()
                .map(|r| r.iter().map(|&d| d as i16).collect())
                .collect();
            let (width, height) = vec2d_size(map);
            let max_i = (height - 1) as i16;
            let max_j = (width - 1) as i16;
            for i in 0..height {
                for j in 0..width {
                    let i_delta = i as i16 + di;
                    let j_delta = j as i16 + dj;
                    let value_at_delta =
                        if i_delta < 0 || i_delta > max_i || j_delta < 0 || j_delta > max_j {
                            Heightmap::MAX
                        } else {
                            *(map
                                .get(i_delta as usize)
                                .unwrap()
                                .get(j_delta as usize)
                                .unwrap())
                        };
                    delta_map[i][j] = value_at_delta as i16 - delta_map[i][j];
                }
            }
            delta_map
        }

        Heightmap {
            delta_up: delta_map(&map, -1, 0),
            delta_right: delta_map(&map, 0, 1),
            delta_down: delta_map(&map, 1, 0),
            delta_left: delta_map(&map, 0, -1),
            map,
        }
    }

    fn lowest_mask(&self) -> Vec<Vec<bool>> {
        let (width, height) = vec2d_size(&self.map);
        let mut is_lowest: Vec<Vec<bool>> = vec![vec![false; width]; height];
        for i in 0..height {
            for j in 0..width {
                is_lowest[i][j] = self.delta_up[i][j] > 0
                    && self.delta_right[i][j] > 0
                    && self.delta_down[i][j] > 0
                    && self.delta_left[i][j] > 0
            }
        }
        is_lowest
    }

    fn print(&self) {
        let lowest_mask = self.lowest_mask();
        for (i, row) in self.map.iter().enumerate() {
            for (j, h) in row.iter().enumerate() {
                print!(
                    "{}{}{}",
                    if lowest_mask[i][j] { '[' } else { ' ' },
                    h,
                    if lowest_mask[i][j] { ']' } else { ' ' }
                );
            }
            println!()
        }
    }

    fn risk_levels_sum(&self) -> u32 {
        let lowest_mask = self.lowest_mask();
        let mut res: u32 = 0;
        for (i, row) in self.map.iter().enumerate() {
            for (j, h) in row.iter().enumerate() {
                if lowest_mask[i][j] {
                    res += (*h as u32) + 1;
                }
            }
        }
        res
    }

    fn basin_higher_from(&self, i: usize, j: usize) -> HashSet<(usize, usize)> {
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        basin.insert((i, j));

        fn move_higher(height: u8, delta: i16) -> bool {
            delta > 0 && (height + (delta as u8) < Heightmap::MAX)
        }

        let height = self.map[i][j];
        if move_higher(height, self.delta_up[i][j]) {
            basin.extend(self.basin_higher_from(i - 1, j).iter());
        }
        if move_higher(height, self.delta_right[i][j]) {
            basin.extend(self.basin_higher_from(i, j + 1).iter());
        }
        if move_higher(height, self.delta_down[i][j]) {
            basin.extend(self.basin_higher_from(i + 1, j).iter());
        }
        if move_higher(height, self.delta_left[i][j]) {
            basin.extend(self.basin_higher_from(i, j - 1).iter());
        }
        basin
    }

    fn basins(&self) -> Vec<HashSet<(usize, usize)>> {
        let lowest_mask = self.lowest_mask();
        let (width, height) = vec2d_size(&self.map);
        let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();

        for i in 0..height {
            for j in 0..width {
                if lowest_mask[i][j] {
                    basins.push(self.basin_higher_from(i, j));
                }
            }
        }
        basins
    }
}

pub fn smoke_basin() {
    let input = read_input(9, false);
    let hm = Heightmap::parse(&input);
    hm.print();

    // part 1
    println!("Total risk level is {}", hm.risk_levels_sum());

    // part 2
    let basins = hm.basins();
    let mut basin_sizes: Vec<usize> = basins.iter().map(|b| b.len()).collect();
    basin_sizes.sort_by(|a, b| b.cmp(a));
    let first_three_sum: usize = basin_sizes.iter().take(3).product();
    println!("Sum of the three larges basin sizes: {}", first_three_sum);
}
