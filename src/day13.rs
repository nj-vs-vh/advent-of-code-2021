use super::utils::read_input;
use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    AlongX(usize),
    AlongY(usize),
}

impl Fold {
    fn parse(s: &str) -> Fold {
        let fold_str = s.split(' ').nth(2).unwrap();
        let fold_coord: usize = fold_str
            .split('=')
            .nth(1)
            .unwrap()
            .parse()
            .expect("Can't parse fold coordinate");
        match fold_str.chars().nth(0).unwrap() {
            'x' => Fold::AlongX(fold_coord),
            'y' => Fold::AlongY(fold_coord),
            _ => panic!("Can't parse orientation"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(s: &str) -> Point {
        let x_y: Vec<usize> = s
            .split(',')
            .map(|s| s.parse().expect("Can't parse coord"))
            .collect();
        Point {
            x: x_y[0],
            y: x_y[1],
        }
    }

    fn folded(&self, fold: &Fold) -> Point {
        match fold {
            Fold::AlongX(fold_pos) => Point {
                y: self.y,
                x: fold_pos - (*fold_pos as i32 - self.x as i32).abs() as usize,
            },
            Fold::AlongY(fold_pos) => Point {
                x: self.x,
                y: fold_pos - (*fold_pos as i32 - self.y as i32).abs() as usize,
            },
        }
    }
}

struct Origami {
    points: HashSet<Point>,
}

impl Origami {
    fn new() -> Origami {
        Origami {
            points: HashSet::new(),
        }
    }

    fn print(&self) {
        let max_x = self.points.iter().map(|p| p.x).max().unwrap() + 1;
        let max_y = self.points.iter().map(|p| p.y).max().unwrap() + 1;
        let mut points_mask: Vec<Vec<bool>> = vec![vec![false; max_x]; max_y];
        for p in &self.points {
            points_mask[p.y][p.x] = true;
        }
        for y in 0..max_y {
            for x in 0..max_x {
                print!("{}", if points_mask[y][x] { '#' } else { '.' });
            }
            println!();
        }
    }

    fn folded(&self, fold: &Fold) -> Origami {
        Origami {
            points: HashSet::from_iter(self.points.iter().map(|p| p.folded(fold))),
        }
    }

    fn count_points(&self) -> usize {
        self.points.len()
    }
}

pub fn folding_origami() {
    let input = read_input(13, false);

    let mut origami: Origami = Origami::new();
    let mut folds: Vec<Fold> = Vec::new();
    let mut parsing_points = true;
    for line in input.lines() {
        if line.len() == 0 {
            parsing_points = false;
            continue;
        }
        if parsing_points {
            origami.points.insert(Point::parse(line));
        } else {
            folds.push(Fold::parse(line));
        }
    }

    // origami.print();
    println!();
    for (i, fold) in folds.iter().enumerate() {
        origami = origami.folded(fold);
        // origami.print();
        println!("points after fold {}: {}", i + 1, origami.count_points());
    }
    origami.print();
}
