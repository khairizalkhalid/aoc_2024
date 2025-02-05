use crate::utils;

use super::day3_part1::sum_of_muls;

fn sum_of_doos(contents: &str) -> i32 {
    let mut doos: Vec<&str> = Vec::new();
    let splitted: Vec<&str> = contents.split("don't()").collect();
    doos.push(splitted.get(0).unwrap());

    for part in splitted.iter().skip(1) {
        doos.extend(part.split("do()").skip(1));
    }

    sum_of_muls(doos.join("").as_str())
}

pub fn run() {
    // from part 1 filter for do until do_not
    // start capture until found do_not
    // continue capture again after matched do
    // send this string unto sum_of_muls
    match utils::file_reader::read_file("day3.txt") {
        Ok(contents) => println!("{:?}", sum_of_doos(&contents)),
        Err(e) => println!("Err: {}", e),
    }
}
