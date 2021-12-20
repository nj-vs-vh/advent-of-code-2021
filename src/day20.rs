use super::utils::read_input;

type Algorithm = [bool; 512];

const MASK: [[usize; 3]; 3] = [[256, 128, 64], [32, 16, 8], [4, 2, 1]];

fn ch2bool(ch: char) -> bool {
    match ch {
        '#' => true,
        _ => false,
    }
}

fn bool2ch(b: bool) -> char {
    match b {
        true => '#',
        false => '.',
    }
}

fn print_algo(a: &Algorithm) {
    for b in a {
        print!("{}", bool2ch(*b));
    }
    println!();
}

struct Image {
    center: Vec<Vec<bool>>,
    padding: bool,
}

impl Image {
    fn parse(s: &String) -> Image {
        let mut img: Vec<Vec<bool>> = Vec::new();
        for line in s.lines().skip(2) {
            let mut row: Vec<bool> = Vec::new();
            for ch in line.chars() {
                row.push(ch2bool(ch))
            }
            img.push(row);
        }
        Image {
            center: img,
            padding: false,
        }
    }

    fn hw(&self) -> (usize, usize) {
        (self.center.len(), self.center.iter().next().unwrap().len())
    }

    fn get(&self, i: i32, j: i32) -> bool {
        let (h, w) = self.hw();
        let h = h as i32;
        let w = w as i32;
        if i >= 0 && i < w && j >= 0 && j < h {
            self.center[i as usize][j as usize]
        } else {
            self.padding
        }
    }

    fn enhanced(&self, algorithm: &Algorithm) -> Image {
        let (h, w) = self.hw();
        let h = h as i32;
        let w = w as i32;
        let mut new_center: Vec<Vec<bool>> = Vec::new();

        for i in -1..(h + 1) {
            let mut new_row: Vec<bool> = Vec::new();
            for j in -1..(w + 1) {
                let mut alg_idx: usize = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if self.get(i + di, j + dj) {
                            // println!("{}, {} true (window center {}, {})", i + di, j + dj, i, j);
                            alg_idx += MASK[(di + 1) as usize][(dj + 1) as usize];
                        }
                    }
                }
                new_row.push(algorithm[alg_idx]);
            }
            new_center.push(new_row);
        }

        let new_padding = match self.padding {
            true => algorithm[511],
            false => algorithm[0],
        };

        Image {
            center: new_center,
            padding: new_padding,
        }
    }

    fn count_lit_pixels(&self) -> Option<usize> {
        // None means infinite
        if self.padding {
            return None;
        }
        let mut counter: usize = 0;
        for row in &self.center {
            for p in row {
                if *p {
                    counter += 1;
                }
            }
        }
        Some(counter)
    }

    fn display(&self) {
        let pad_len: usize = 2;
        let (_, w) = self.hw();
        for _ in 0..pad_len {
            (0..(w + 2 * pad_len)).for_each(|_| print!("{}", bool2ch(self.padding)));
            println!();
        }
        for row in &self.center {
            (0..pad_len).for_each(|_| print!("{}", bool2ch(self.padding)));
            for v in row {
                print!("{}", bool2ch(*v))
            }
            (0..pad_len).for_each(|_| print!("{}", bool2ch(self.padding)));
            println!();
        }
        for _ in 0..pad_len {
            (0..(w + 2 * pad_len)).for_each(|_| print!("{}", bool2ch(self.padding)));
            println!();
        }
    }
}

pub fn image_enhancement() {
    let input = read_input(20, false);

    let algorithm: Algorithm = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|ch| ch == '#')
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();

    let mut image = Image::parse(&input);

    print_algo(&algorithm);

    image.display();
    println!();
    for _ in 0..50 {
        image = image.enhanced(&algorithm)
    }

    image.display();

    if let Some(count) = image.count_lit_pixels() {
        println!("pixels lit: {}", count);
    }
}
