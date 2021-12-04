mod day01;
mod day02;
mod day03;
mod day04;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day_to_run: u8 = (&args[1]).parse().expect("Command line argument must be an integer");
        match day_to_run {
            1 => {day01::sonar();}
            2 => {day02::submarine_path()}
            3 => {day03::submarine_ratings()}
            _ => {println!("Day {} is not yet implemented", day_to_run)}
        }
    }
    else {
        day04::squid()
    }
}
