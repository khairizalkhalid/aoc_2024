use std::{
    env,
    io::{stdout, Write},
};

use curl::easy::Easy;

// create a utility to download aoc input files and store in input folder as a txt file
// if args size 2 and args[1] = 'get_input' ie: 'cargo run get_input 6
// download file using curl into input/dayX.txt
// `curl -v --cookie "session=AOC24_COOKIE" https://adventofcode.com/2024/day/7/input > input/day7.txt`
// store session cookie in system environment AOC24_COOKIE
pub fn run(day: &str) {
    let mut easy = Easy::new();
    let url = "https://adventofcode.com/2024/day/".to_string() + day + "/input";

    match env::var("AOC24_COOKIE") {
        Ok(cookie) => {
            let set_cooker_header = "session=".to_string() + &cookie;
            easy.url(&url).unwrap();
            easy.cookie(&set_cooker_header).unwrap();
            easy.write_function(|data| {
                stdout().write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();
            easy.perform().unwrap()
        }
        Err(_) => eprintln!("AOC24_COOKIE not found in environment variable!"),
    }
}
