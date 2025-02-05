use crate::utils;

use super::day3_part1::sum_of_muls;

fn sum_of_doos(contents: &str) -> i32 {
    let mut doos: Vec<&str> = Vec::new();
    let splited: Vec<_> = contents.split("don't()").collect();
    doos.push(splited.get(0).unwrap());
    println!("{:?}", doos);
    sum_of_muls(doos.join("").as_str())
}

pub fn run() {
    // from part 1 filter for do until do_not
    // start capture until found do_not
    // continue capture again after matched do
    // send this string unto sum_of_muls
    let test_case = "xmul(2,4)%&mul[3,7]!@^don't()_mul(5,5)+mul(32,64]then(mul(11,8)do()mul(8,5))";
    println!("{:?}", sum_of_doos(test_case));
    //match utils::file_reader::read_file("day3.txt") {
    //    Ok(contents) => println!("{:?}", sum_of_doos(&contents)),
    //    Err(e) => println!("Err: {}", e),
    //}
}
