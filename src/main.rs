use std::env;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprint!("Please provide which day and part to run. (e.g: `cargo run d1p1`)\n");
        return;
    }

    let day = &args[1];
    match day.as_str() {
        "d1p1" => days::day1_part1::run(),
        "d1p2" => days::day1_part2::run(),
        _ => eprint!("Program {} is not available.\n", day),
    }
}
