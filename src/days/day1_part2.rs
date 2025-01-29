use std::i32;

use crate::{days::day1_part1, utils};

pub fn run() {
    // first index: iterate thru to find equal number (call it cur)
    // found, add to counter
    // finally multiply cur with the counter call it score
    // replace cur with score
    // repeate for each vec1...
    // the end sum all vec1
    match utils::file_reader::read_file("day1.txt") {
        Ok(contents) => {
            let (vec1, vec2) = day1_part1::split_contents_into_two_vectors(&contents);
            let total: i32 = vec1
                .iter()
                .map(|&a| a * vec2.iter().filter(|&&b| b == a).count() as i32)
                .sum();
            println!("{:?}", total)
        }
        Err(e) => println!("Error: {}", e),
    }
}
