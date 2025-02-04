use crate::utils;

use super::day3_part1::sum_of_muls;

pub fn run() {
    // from part 1 filter for do until do_not
    // start capture until found do_not
    // continue capture again after matched do
    // send this string unto sum_of_muls
    match utils::file_reader::read_file("day3.txt") {
        Ok(contents) => println!("{:?}", sum_of_muls(&contents)),
        Err(e) => println!("Err: {}", e),
    }
}
