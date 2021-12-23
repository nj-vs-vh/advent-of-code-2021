use super::utils::read_input;
use bit_set::BitSet;
use itertools::Itertools;

fn parse_cuboid(s: &str) -> (bool, [i32; 6]) {
    let mut s_iter = s.split(" ");
    let polarity: bool = match s_iter.next().unwrap() {
        "on" => true,
        "off" => false,
        _ => panic!("Can't parse polarity from {}", s),
    };
    fn parse_bounds(s: &str) -> [i32; 2] {
        let s: String = String::from(&s[2..]);
        let mut bounds_iter = s.split("..");
        [
            bounds_iter
                .next()
                .unwrap()
                .parse()
                .expect("Can't parse lower bound"),
            bounds_iter
                .next()
                .unwrap()
                .parse()
                .expect("Can't parse upper bound"),
        ]
    }

    let coords_str = String::from(s_iter.next().unwrap());
    let mut coords: [i32; 6] = [0, 0, 0, 0, 0, 0];
    for (i, bounds_str) in coords_str.split(",").enumerate() {
        let bounds = parse_bounds(bounds_str);
        coords[2 * i] = bounds[0];
        coords[2 * i + 1] = bounds[1];
    }
    (polarity, coords)
}

#[derive(Debug)]
struct CuboidBound {
    coord: i32,
    cuboid_idx: usize,
    opening: bool,
}

fn to_bound_sequence(cuboids: &Vec<[i32; 6]>, dim: usize) -> Vec<CuboidBound> {
    let get_lu = |cub: &[i32; 6]| (cub[2 * dim], cub[2 * dim + 1]);
    let mut cbs: Vec<CuboidBound> = Vec::new();
    for (i, c) in cuboids.iter().enumerate() {
        let (low, upp) = get_lu(c);
        cbs.push(CuboidBound {
            cuboid_idx: i,
            coord: low,
            opening: true,
        });
        cbs.push(CuboidBound {
            cuboid_idx: i,
            coord: upp + 1,
            opening: false,
        });
    }
    cbs.sort_by_key(|cb| cb.coord);
    cbs
}

#[allow(dead_code)]
fn clamp_cuboid(c: &[i32; 6], max_abs: i32) -> [i32; 6] {
    let clamp = |v: i32, add: i32| {
        if v > max_abs {
            max_abs + add
        } else if v < -max_abs {
            -max_abs + add
        } else {
            v
        }
    };

    let mut res: [i32; 6] = [0, 0, 0, 0, 0, 0];
    for dim in 0..3 {
        res[2 * dim] = clamp(c[2 * dim], 1);
        res[2 * dim + 1] = clamp(c[2 * dim + 1], 0);
    }
    res
}

pub fn reactor_cubes() {
    let input = read_input(22, false);

    let cubspecs: Vec<(bool, [i32; 6])> = input.lines().map(|l| parse_cuboid(l)).collect();

    let cuboids: Vec<[i32; 6]> = cubspecs.iter().map(|c| c.1).collect();

    // for pt1
    // let cuboids: Vec<[i32; 6]> = cuboids.iter().map(|c| clamp_cuboid(c, 50)).collect();

    let polarities: Vec<bool> = cubspecs.iter().map(|c| c.0).collect();

    let x_bounds = to_bound_sequence(&cuboids, 0);
    let y_bounds = to_bound_sequence(&cuboids, 1);
    let z_bounds = to_bound_sequence(&cuboids, 2);

    let mut open_along_x: BitSet = BitSet::new();
    let mut open_along_y: BitSet = BitSet::new();
    let mut open_along_z: BitSet = BitSet::new();

    fn track_open_cuboids(open_set: &mut BitSet, b: &CuboidBound) {
        if b.opening {
            open_set.insert(b.cuboid_idx);
        } else {
            open_set.remove(b.cuboid_idx);
        }
    }

    let mut total_on: u64 = 0;
    for (x_l, x_r) in x_bounds.iter().tuple_windows() {
        track_open_cuboids(&mut open_along_x, x_l);
        open_along_y.clear();
        for (y_l, y_r) in y_bounds
            .iter()
            .filter(|yb| open_along_x.contains(yb.cuboid_idx))
            .tuple_windows()
        {
            track_open_cuboids(&mut open_along_y, y_l);
            open_along_z.clear();
            for (z_l, z_r) in z_bounds
                .iter()
                .filter(|zb| open_along_y.contains(zb.cuboid_idx))
                .tuple_windows()
            {
                track_open_cuboids(&mut open_along_z, z_l);

                if let Some(latest_open_cuboid_idx) = open_along_z.iter().max() {
                    if polarities[latest_open_cuboid_idx] {
                        let dx = (x_r.coord - x_l.coord) as u64;
                        let dy = (y_r.coord - y_l.coord) as u64;
                        let dz = (z_r.coord - z_l.coord) as u64;
                        total_on += dx * dy * dz;
                    }
                }
            }
        }
    }

    println!("{}", total_on);
}
