use std::fs;

fn count_combinations(n: usize, r: usize) -> usize {
    if r > n {
        0
    } else {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

fn count_lanterfish(t: usize) -> usize {
    let a: usize = 7;
    let b: usize = 9;
    assert_eq!(a < b, true);
    let k_l_max = (t as f32 / a as f32).ceil() as usize;
    let k_r_max = (t as f32 / b as f32).ceil() as usize;
    let mut k_lr_table = vec![vec![0; k_r_max + 1]; k_l_max + 1];
    for k_l in 0..=k_l_max {
        for k_r in 0..=k_r_max {
            let expected_day_of_fork = a * k_l + b * k_r;
            k_lr_table[k_l][k_r] = expected_day_of_fork;
            if expected_day_of_fork >= t {
                break;
            }
        }
    }

    // for row in &k_lr_table {
    //     for el in row {
    //         print!("{:#3}", el);
    //     }
    //     println!("");
    // }
    let mut total: usize = 0;
    for k_l in 0..=k_l_max {
        for k_r in 0..=k_r_max {
            if k_lr_table[k_l][k_r] >= t {
                if let Some(down_row) = k_lr_table.get(k_l + 1) {
                    if down_row[k_r] != 0 {
                        break;
                    }
                }
                // println!("({}, {}): {}", k_l, k_r, k_lr_table[k_l][k_r]);
                total += count_combinations(k_l + k_r, k_l);
                break;
            }
        }
    }

    total
}

pub fn lanternfish() {
    let input = fs::read_to_string("data/day06/input.txt").expect("Can't read input file");
    let n0s: Vec<usize> = input
        .lines()
        .next()
        .expect("Input must contain at least one line")
        .split(',')
        .map(|dts| dts.parse().expect("Unable to parse integer"))
        .collect();

    for n_days in [80, 256] {
        let mut total: usize = 0;
        for n0 in &n0s {
            total += count_lanterfish(n_days - n0);
        }
        println!("After {} days: {}", n_days, total);
    }
}
