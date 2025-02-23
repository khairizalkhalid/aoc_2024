use crate::utils;

#[derive(PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(PartialEq, Eq)]
enum Direction {
    None,
    Positive,
    Negative,
}

pub fn get_readings_vector(contents: &str) -> Vec<Vec<i32>> {
    contents
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse to integer"))
                .collect()
        })
        .collect()
}

pub fn safety_check(readings: Vec<i32>) -> bool {
    let mut direction: Direction = Direction::None;
    let mut safety: Safety = Safety::Safe;
    for window in readings.windows(2) {
        let r = window[0];
        let next_r = window[1];
        let safe_range: i32 = r - next_r;
        let abs_range = safe_range.abs();

        if abs_range > 3 || abs_range == 0 {
            safety = Safety::Unsafe;
            break;
        }

        if direction == Direction::None {
            direction = if r > next_r {
                Direction::Positive
            } else {
                Direction::Negative
            }
        } else if (direction == Direction::Negative && r > next_r)
            || (direction == Direction::Positive && next_r > r)
        {
            safety = Safety::Unsafe;
            break;
        }
    }
    safety == Safety::Safe
}

pub fn test_run() {
    let test_case = vec![
        vec![7, 6, 4, 2, 1], //Safe because the levels are all decreasing by 1 or 2.
        vec![1, 2, 7, 8, 9], //Unsafe because 2 7 is an increase of 5.
        vec![9, 7, 6, 2, 1], //Unsafe because 6 2 is a decrease of 4.
        vec![1, 3, 2, 4, 5], //Unsafe because 1 3 is increasing but 3 2 is decreasing.
        vec![8, 6, 4, 4, 1], //Unsafe because 4 4 is neither an increase or a decrease.
        vec![1, 3, 6, 7, 9], //Safe because the levels are all increasing by 1, 2, or 3.
    ];

    let result: Vec<bool> = test_case
        .iter()
        .map(|vec_z| safety_check(vec_z.to_vec()))
        .collect();
    println!("{:?}", result);
    println!("{:?}", result.iter().filter(|&&r| r).count());
}

pub fn run() {
    // convert data into a X of Y
    // repeat for each Y
    // push safetyCheck result into Z
    // safetyCheck:
    // - loop thru X until max-1
    // - calculate value of cur index minus (cur + 1)
    // - set the result in a binding called safeRange
    // - abs safeRange > 2 : return unsafe
    // - set a binding called isIncreasing with a case:
    // - - > 0 : true
    // - - = 0 : return unsafe
    // - - < 0 : false
    // - second loop: check (isIncreasing && safeRange < 0) || (!isIncreasing && safeRange > 0): return unsafe
    // - return safe
    // count safe in Z
    match utils::file_reader::read_file("day2.txt") {
        Ok(contents) => {
            let readings_vector = get_readings_vector(&contents);
            let result: Vec<bool> = readings_vector
                .iter()
                .map(|vec_z| safety_check(vec_z.to_vec()))
                .collect();
            println!("{:?}", result.iter().filter(|&&r| r).count());
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_readings_vector() {
        let contents = "1 2 3 4 5\n5 4 3 2 1";
        let readings_vector = get_readings_vector(contents);
        assert_eq!(
            readings_vector,
            vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]]
        )
    }

    #[test]
    fn test_safety_check() {
        assert_eq!(true, safety_check(vec![1, 2, 3, 4, 5])); // safe: increase max 1
        assert_eq!(true, safety_check(vec![1, 2, 3, 4, 6])); // safe: increase max 2
        assert_eq!(true, safety_check(vec![1, 2, 3, 4, 7])); // safe: increase max 3
        assert_eq!(true, safety_check(vec![5, 4, 3, 2, 1])); // safe: decrease max 1
        assert_eq!(true, safety_check(vec![6, 4, 3, 2, 1])); // safe: decrease max 2
        assert_eq!(true, safety_check(vec![7, 4, 3, 2, 1])); // safe: decrease max 3
        assert_eq!(false, safety_check(vec![1, 2, 3, 4, 8])); // unsafe: increase more than 3
        assert_eq!(false, safety_check(vec![1, 2, 3, 2, 5])); // unsafe: increase then decrease
        assert_eq!(false, safety_check(vec![1, 1, 3, 4, 5])); // unsafe: neither increase/decrease
        assert_eq!(false, safety_check(vec![2, 1, 3, 4, 5])); // unsafe: deacrease
    }
}
