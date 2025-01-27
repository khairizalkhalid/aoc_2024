use crate::days::day1_part1;
use std::i32;

pub fn run() {
    // first index: iterate thru to find equal number (call it cur)
    // found, add to counter
    // finally multiply cur with the counter call it score
    // replace cur with score
    // repeate for each vec1...
    // the end sum all vec1
    match day1_part1::read_file() {
        Ok(contents) => {
            let mut final_vec: Vec<i32> = Vec::new();
            let (vec1, vec2) = day1_part1::split_contents_into_two_vectors(&contents);
            for cur in vec1 {
                let count = vec2.iter().filter(|a| **a == cur).count() as i32;
                final_vec.push(cur * count)
            }

            let total: i32 = final_vec.iter().sum();
            println!("{:?}", total)
        }
        Err(e) => println!("Error: {}", e),
    }
}
