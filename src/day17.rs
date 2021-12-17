use super::utils::read_input;
use std::collections::{HashMap, HashSet};

fn calc_v_range(y_range: &(i32, i32), t: i32) -> (f32, f32) {
    let y_min = y_range.0 as f32;
    let y_max = y_range.1 as f32;
    let t = t as f32;

    fn y2vy(y: f32, t: f32) -> f32 {
        (2.0 * y / t + t - 1.0) / 2.0
    }

    (y2vy(y_min, t), y2vy(y_max, t))
}

fn int_vy_in_range(vy_range: &(f32, f32)) -> Vec<i32> {
    let mut vy_int = vy_range.0.ceil();
    let mut res: Vec<i32> = Vec::new();
    while vy_int <= vy_range.1 {
        res.push(vy_int as i32);
        vy_int += 1.0;
    }
    res
}

pub fn probe_launch() {
    let input = read_input(17, false);

    fn parse_range(s: &str) -> (i32, i32) {
        let mut it = s.split("..").map(|ss| ss.parse().expect(""));
        (it.next().unwrap(), it.next().unwrap())
    }

    let mut ranges = input.split("x=").nth(1).unwrap().split(", y=");
    let x_range = parse_range(ranges.next().unwrap());
    let y_range = parse_range(ranges.next().unwrap());

    let mut t: i32 = 0;

    let mut t_by_vy: HashMap<i32, Vec<i32>> = HashMap::new();

    loop {
        t += 1;
        let vy_range = calc_v_range(&y_range, t); // vy range to end up in y range after t

        let int_vys = int_vy_in_range(&vy_range);
        println!("{}: {:?}, vy = {:?}", t, vy_range, int_vys);
        for int_vy in int_vys {
            if !t_by_vy.contains_key(&int_vy) {
                t_by_vy.insert(int_vy, Vec::new());
            }
            t_by_vy.get_mut(&int_vy).unwrap().push(t);
        }
        if t > 1000 {
            // TEMP
            break;
        }
    }

    println!("\n\n{:?}\n\n", t_by_vy);

    let mut vectors: HashSet<(i32, i32)> = HashSet::new();

    for (vy, t_variants) in t_by_vy.iter() {
        for t in t_variants {
            let mut vx: i32 = 0;
            loop {
                vx += 1;
                let vx_t = [vx, *t - 1];
                let tx = 1 + (vx_t).iter().min().unwrap();
                let x_coord = (tx * vx - ((tx - 1) * tx) / 2) as i32;
                if x_coord >= x_range.0 && x_coord <= x_range.1 {
                    if tx != *t {}
                    println!("{} ({}): v = ({}, {})", t, tx, vx, vy);
                    vectors.insert((vx, *vy));
                } else if x_coord > 5 * x_range.1 {
                    break;
                }
            }
        }
    }

    println!("{:?}", vectors.len());
}
