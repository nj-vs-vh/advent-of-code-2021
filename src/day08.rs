use std::fs;

pub fn messed_up_displays() {
    let input = fs::read_to_string("data/day08/input.txt").expect("Can't read input file");

    let entries: Vec<&str> = input.lines().collect();

    let mut length_discriminatable_digits_total: usize = 0;
    for entry in &entries {
        let digits_reading = entry.split(" | ").nth(1).expect("Can't get digits");
        let digit_lens: Vec<usize> = digits_reading.split(" ").map(|s| s.len()).collect();
        // println!("{:?}", digit_lens);
        length_discriminatable_digits_total += digit_lens
            .iter()
            .filter(|&&l| l == 2 || l == 3 || l == 4 || l == 7)
            .count()
    }

    println!(
        "We can distinguish {} letters in total",
        length_discriminatable_digits_total
    );
}
