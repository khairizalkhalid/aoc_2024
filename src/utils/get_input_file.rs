// create a utility to download aoc input files and store in input folder as a txt file
// if args size 2 and args[1] = 'get_input' ie: 'cargo run get_input 6
// download file using curl into input/dayX.txt
// `curl -v --cookie "session=AOC24_COOKIE" https://adventofcode.com/2024/day/7/input > input/day7.txt`
// store session cookie in system environment AOC24_COOKIE
pub fn run(day: &str) {
    println!("this is sparta {}", day)
}
