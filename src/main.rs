mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day_to_run: u8 = (&args[1])
            .parse()
            .expect("Command line argument must be an integer");
        match day_to_run {
            1 => day01::sonar(),
            2 => day02::submarine_path(),
            3 => day03::submarine_ratings(),
            4 => day04::squid(),
            5 => day05::vent_lines(),
            6 => day06::lanternfish(),
            7 => day07::align_crabs(),
            8 => day08::messed_up_displays(),
            9 => day09::smoke_basin(),
            10 => day10::brackets_parsing(),
            11 => day11::dumbos(),
            12 => day12::path_in_caves(),
            13 => day13::folding_origami(),
            14 => day14::polymers(),
            15 => day15::chitons(),
            16 => day16::bits_decoding(),
            17 => day17::probe_launch(),
            18 => day18::snailfish_math(),
            19 => day19::beacons(),
            20 => day20::image_enhancement(),
            21 => day21::dirac_die(),
            22 => day22::reactor_cubes(),
            23 => day23::amphipod_rooms(),
            _ => {
                println!("Day {} is not yet implemented", day_to_run)
            }
        }
    } else {
        day24::arithmetic_logic_unit();
        // new day will be here...
    }
}
