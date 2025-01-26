use std::env;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprint!("Please provide which day to run. (e.g: `cargo run 1`)");
        return;
    }

    let day = &args[1];
    match day.as_str() {
        "1" => days::day1::run(),
        _ => eprint!("Day {} is not available.", day),
    }
}
