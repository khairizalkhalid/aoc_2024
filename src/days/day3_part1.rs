use regex::Regex;

use crate::utils;

fn sum_of_muls(mul_string: &str) -> i32 {
    let reg = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let reg_matches: Vec<_> = reg.find_iter(mul_string).map(|m| m.as_str()).collect();

    let sum_of_products: i32 = reg_matches
        .iter()
        .filter_map(|m| {
            m.strip_prefix("mul(")
                .and_then(|s| s.strip_suffix(")"))
                .map(|s| {
                    s.split(',')
                        .map(|n| n.parse::<i32>().unwrap())
                        .product::<i32>()
                })
        })
        .sum();

    sum_of_products
}

pub fn test_run() {
    let test_case = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    println!("{:?}", sum_of_muls(test_case));
}

pub fn run() {
    // split string by regex exp `mul\(\d+,\d+\)`
    // for each set of string
    // remove the word 'mul'
    // split using comma, first item remove infront while the second remove the end
    // parse to i32
    // calculate multiplication between the two i32
    // push to a new vec
    // then iter sum it up.
    match utils::file_reader::read_file("day3.txt") {
        Ok(contents) => println!("{:?}", sum_of_muls(&contents)),
        Err(e) => println!("Err: {}", e),
    }
}
