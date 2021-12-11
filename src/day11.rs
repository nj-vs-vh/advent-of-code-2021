use super::utils::read_input;
use std::convert::TryInto;

const CAVE_SIZE: usize = 10;

struct DumboCave {
    energy: [[u8; CAVE_SIZE]; CAVE_SIZE],
}

impl DumboCave {
    fn parse(s: &String) -> DumboCave {
        let energies_vvec: Vec<Vec<u8>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        let enery_ar_vec: Vec<[u8; CAVE_SIZE]> = energies_vvec
            .into_iter()
            .map(|v| v.try_into().unwrap())
            .collect();
        DumboCave {
            energy: enery_ar_vec.try_into().unwrap(),
        }
    }

    fn print(&self) {
        println!();
        for i in 0..CAVE_SIZE {
            for j in 0..CAVE_SIZE {
                print!(
                    "{}{}{}",
                    if self.energy[i][j] == 0 { '[' } else { ' ' },
                    self.energy[i][j],
                    if self.energy[i][j] == 0 { ']' } else { ' ' }
                );
            }
            println!();
        }
    }

    fn step(&mut self) -> u32 {
        let mut has_flashed = [[false; CAVE_SIZE]; CAVE_SIZE];
        for i in 0..CAVE_SIZE {
            for j in 0..CAVE_SIZE {
                self.energy[i][j] += 1;
            }
        }

        fn sum_bool_mask(mask: &[[bool; CAVE_SIZE]; CAVE_SIZE]) -> u32 {
            let mut sum: u32 = 0;
            for i in 0..CAVE_SIZE {
                for j in 0..CAVE_SIZE {
                    if mask[i][j] {
                        sum += 1;
                    }
                }
            }
            sum
        }

        let mut flashed_total: u32 = 0;
        loop {
            let flashed_before = sum_bool_mask(&has_flashed);
            for i in 0..CAVE_SIZE {
                for j in 0..CAVE_SIZE {
                    if !has_flashed[i][j] && self.energy[i][j] > 9 {
                        has_flashed[i][j] = true;
                        self.energy[i][j] = 0;
                        for di in -1..=1 {
                            for dj in -1..=1 {
                                let (i2, j2) = (i as i8 + di, j as i8 + dj);
                                if i2 < 0 || j2 < 0 {
                                    continue;
                                }
                                let (i2, j2) = (i2 as usize, j2 as usize);
                                if i2 >= CAVE_SIZE || j2 >= CAVE_SIZE {
                                    continue;
                                }
                                if !has_flashed[i2][j2] {
                                    self.energy[i2][j2] += 1;
                                }
                            }
                        }
                    }
                }
            }
            let flashed_after = sum_bool_mask(&has_flashed);
            if flashed_after == flashed_before {
                break;
            } else {
                flashed_total += flashed_after - flashed_before
            }
        }
        flashed_total
    }
}

pub fn dumbos() {
    let input = read_input(11, false);

    let mut dc = DumboCave::parse(&input);
    // dc.print();
    let mut flashed_total: u32 = 0;
    for _ in 0..100 {
        flashed_total += dc.step();
    }
    dc.print();
    println!("Total flashes: {}", flashed_total);

    let mut unsync_step_count: usize = 100;
    loop {
        if dc.step() == ((CAVE_SIZE * CAVE_SIZE) as u32) {
            break;
        } else {
            unsync_step_count += 1
        }
    }
    dc.print();
    println!("Synchronized flash as step: {}", unsync_step_count + 1);
}
