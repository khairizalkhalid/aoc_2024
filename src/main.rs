use std::collections::HashMap;
use std::env;
use std::time::Instant;

mod days;
mod utils;

fn time_elapsed_wrapper<F>(proc: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    proc();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Please provide which day and part to run. (e.g: `cargo run d1p1`, `cargo input 1`)"
        );
        return;
    }

    if &args[1] == "input" && args.len() == 3 {
        utils::get_input_file::run(&args[2]);
        return;
    }

    let day = &args[1];

    let mut functions: HashMap<&str, fn()> = HashMap::new();
    functions.insert("d1p1", || days::day1_part1::run());
    functions.insert("d1p2", || days::day1_part2::run());
    functions.insert("d2t", || days::day2_part1::test_run());
    functions.insert("d2p1", || days::day2_part1::run());
    functions.insert("d2p2", || days::day2_part2::run());
    functions.insert("d3pt", || days::day3_part1::test_run());
    functions.insert("d3p1", || days::day3_part1::run());
    functions.insert("d3p2", || days::day3_part2::run());
    functions.insert("d4pt", || days::day4_part1::test_run());
    functions.insert("d4p1", || days::day4_part1::run());
    functions.insert("d4p2", || days::day4_part2::run());
    functions.insert("d5p1", || days::day5_part1::run());

    if let Some(&function) = functions.get(day.as_str()) {
        time_elapsed_wrapper(function);
    } else {
        eprintln!("Program {} is not available.", day);
    }
}
