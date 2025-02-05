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
        eprintln!("Please provide which day and part to run. (e.g: `cargo run d1p1`)");
        return;
    }

    let day = &args[1];
    match day.as_str() {
        "d1p1" => time_elapsed_wrapper(|| days::day1_part1::run()),
        "d1p2" => time_elapsed_wrapper(|| days::day1_part2::run()),
        "d2t" => time_elapsed_wrapper(|| days::day2_part1::test_run()),
        "d2p1" => time_elapsed_wrapper(|| days::day2_part1::run()),
        "d2p2" => time_elapsed_wrapper(|| days::day2_part2::run()),
        "d3pt" => time_elapsed_wrapper(|| days::day3_part1::test_run()),
        "d3p1" => time_elapsed_wrapper(|| days::day3_part1::run()),
        "d3p2" => time_elapsed_wrapper(|| days::day3_part2::run()),
        _ => eprintln!("Program {} is not available.", day),
    }
}
