use crate::{
    days::day2_part1::{get_readings_vector, safety_check},
    utils,
};

fn enhanced_safety_check(readings: &Vec<i32>) -> bool {
    // 1) [1, 2, 3, 4, 5, 6]
    // 2) check this reading, if succ, return safe
    // 3) if fail remove x index, repeat 2
    // 4) return unsafe
    if safety_check(readings.to_vec()) {
        return true;
    }
    // TODO: do a iterator on this reading to remove each index and run safety_check again
    safety_check(readings.to_vec())
}

pub fn run() {
    // same logic as previous task
    // wrap another function on top of the prevouse safe chec with this:
    // if safe, return safe
    // then check safe unsafe again on the new stack with one level removed
    match utils::file_reader::read_file("day2.txt") {
        Ok(contents) => {
            let readings_vector = get_readings_vector(&contents);
            let result: Vec<bool> = readings_vector
                .iter()
                .map(|vec_z| enhanced_safety_check(&vec_z.to_vec()))
                .collect();
            println!("{:?}", result.iter().filter(|&&r| r).count());
        }
        Err(e) => println!("Error: {}", e),
    }
}
