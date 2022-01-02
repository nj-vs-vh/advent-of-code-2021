use std::fs;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(s: &str) -> Point {
        let mut coords = s.split(",").map(|s| {
            s.parse::<usize>()
                .expect("Can't parse integer value from point spec")
        });
        Point {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_hv(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn is_45deg(&self) -> bool {
        fn diff(v1: usize, v2: usize) -> usize {
            (v1 as i32 - v2 as i32).abs() as usize
        }

        diff(self.start.x, self.end.x) == diff(self.start.y, self.end.y)
    }

    fn parse(s: &str) -> Line {
        let mut points = s.split(" -> ").map(Point::parse);
        Line {
            start: points.next().unwrap(),
            end: points.next().unwrap(),
        }
    }

    fn ends(&self) -> Vec<&Point> {
        vec![&self.start, &self.end]
    }

    fn points(&self) -> Vec<Point> {
        fn range_ordered(v1: usize, v2: usize) -> Vec<usize> {
            match v2 > v1 {
                true => (v1..=v2).collect(),
                false => (v2..=v1).rev().collect(),
            }
        }

        if self.is_hv() {
            if self.start.x == self.end.x {
                Vec::from_iter(
                    range_ordered(self.start.y, self.end.y)
                        .iter()
                        .map(|&y| Point { x: self.start.x, y }),
                )
            } else if self.start.y == self.end.y {
                Vec::from_iter(
                    range_ordered(self.start.x, self.end.x)
                        .iter()
                        .map(|&x| Point { x, y: self.start.y }),
                )
            } else {
                panic!("Error in is_hv method!");
            }
        } else if self.is_45deg() {
            range_ordered(self.start.x, self.end.x)
                .iter()
                .zip(range_ordered(self.start.y, self.end.y).iter())
                .map(|(&x, &y)| Point { x, y })
                .collect()
        } else {
            panic!("Can only iterate over points of horizontal, vertical or 45 degree lines")
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LineField<'a> {
    lines: Vec<&'a Line>,
    raster: Vec<Vec<u32>>,
    x_range: (usize, usize), // including both ends
    y_range: (usize, usize),
}

impl LineField<'_> {
    fn from_lines<'a>(lines: Vec<&'a Line>) -> LineField<'a> {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (
            lines[0].start.x,
            lines[0].start.y,
            lines[0].start.x,
            lines[0].start.y,
        );
        // very ugly, how to rewrite with iterators?
        for line in &lines {
            for pt in line.ends() {
                if pt.x < min_x {
                    min_x = pt.x;
                }
                if pt.x > max_x {
                    max_x = pt.x;
                }
                if pt.y < min_y {
                    min_y = pt.y;
                }
                if pt.y > max_y {
                    max_y = pt.y;
                }
            }
        }
        let mut raster: Vec<Vec<u32>> = vec![vec![0; max_x - min_x + 1]; max_y - min_y + 1];

        for line in &lines {
            for internal_point in line.points() {
                raster[internal_point.y - min_y][internal_point.x - min_x] += 1;
            }
        }

        LineField {
            lines: lines.clone(),
            raster,
            x_range: (min_x, max_x),
            y_range: (min_y, max_x),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.raster {
            for val in row {
                print!(
                    "{}",
                    if *val == 0 {
                        String::from(".")
                    } else {
                        format!("{}", val)
                    }
                )
            }
            println!()
        }
    }

    fn count_overlaps(&self) -> u32 {
        let mut count = 0;
        for row in &self.raster {
            for val in row {
                if *val > 1 {
                    count += 1
                }
            }
        }
        count
    }
}

pub fn vent_lines() {
    let input = fs::read_to_string("data/day05/input.txt").expect("Can't read input file");

    let all_lines: Vec<Line> = input.lines().map(Line::parse).collect();

    // part 1
    let field = LineField::from_lines(all_lines.iter().filter(|l| l.is_hv()).collect());
    // field.print();
    println!("number of overlapping points: {}", field.count_overlaps());

    // part 2
    let field = LineField::from_lines(
        all_lines
            .iter()
            .filter(|l| l.is_hv() || l.is_45deg())
            .collect(),
    );
    // field.print();
    println!("number of overlapping points: {}", field.count_overlaps());
}
