// task: in each line, find which one is correct if "x : a b c" where x must be equal a, b, c using
// operators "+" and "*"
// then add up all the one that is correct.

use crate::utils;

fn is_valid_config(calib: i64, configs: &Vec<i64>) -> bool {
    let permutations = generate_permutations(configs.len());

    for ops in permutations {
        let mut result = configs[0];
        for (i, op) in ops.iter().enumerate() {
            if i + 1 < configs.len() {
                match op {
                    '+' => result += configs[i + 1],
                    '*' => result *= configs[i + 1],
                    _ => unreachable!(),
                }
            }
        }
        if result == calib {
            return true;
        }
    }

    false
}

// function to write plus or multiply permutations based on the number of configs minus 1
// e.g. if configs = [1, 2, 3], then we have 2 operators to choose from
// we can have 2^2 = 4 permutations
// we can use a bitmask to generate the permutations
// ie: 0: [ "+", "+" ]
// 1: [ "+", "*" ]
// 2: [ "*", "+" ]
// 3: [ "*", "*" ]
fn generate_permutations(num_configs: usize) -> Vec<Vec<char>> {
    let total_permutations = 1 << (num_configs - 1); // 2^(num_configs - 1)
    (0..total_permutations)
        .map(|i| {
            (0..(num_configs - 1))
                .map(|j| if (i & (1 << j)) != 0 { '*' } else { '+' })
                .collect()
        })
        .collect()
}

fn get_calibration_and_configs(line: &str) -> Result<(i64, Vec<i64>), String> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid line format: {}", line));
    }
    let calib = parts[0]
        .trim()
        .parse::<i64>()
        .map_err(|_| "Invalid calibration value")?;
    let configs = parts[1]
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().map_err(|_| "Invalid config value"))
        .collect::<Result<Vec<i64>, _>>()?;
    Ok((calib, configs))
}

fn get_total_valid_calibrations(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| match get_calibration_and_configs(line) {
            Ok((calib, configs)) if is_valid_config(calib, &configs) => Some(calib),
            _ => None,
        })
        .sum()
}

pub fn run() {
    match utils::file_reader::read_file("day7.txt") {
        Ok(contents) => {
            let total = get_total_valid_calibrations(contents.as_str());
            println!("Total valid calibrations: {}", total);
        }
        Err(e) => println!("Err: {}", e),
    }
}

#[cfg(test)]
mod test {
    use crate::days::day7_part1::{
        generate_permutations, get_calibration_and_configs, get_total_valid_calibrations,
        is_valid_config,
    };

    #[test]
    fn test_is_valid_config() {
        // 190: 10 19 (true)
        // 3267: 81 40 27 (true)
        // 83: 17 5
        // 156: 15 6
        // 7290: 6 8 6 15
        // 161011: 16 10 13
        // 192: 17 8 14
        // 21037: 9 7 18 13
        // 292: 11 6 16 20 (true)
        assert_eq!(is_valid_config(190, &vec![10, 19]), true);
        assert_eq!(is_valid_config(3267, &vec![81, 40, 27]), true);
        assert_eq!(is_valid_config(83, &vec![17, 5]), false);
        assert_eq!(is_valid_config(156, &vec![15, 6]), false);
        assert_eq!(is_valid_config(7290, &vec![6, 8, 6, 15]), false);
        assert_eq!(is_valid_config(161011, &vec![16, 10, 13]), false);
        assert_eq!(is_valid_config(192, &vec![17, 8, 14]), false);
        assert_eq!(is_valid_config(292, &vec![11, 6, 16, 20]), true);
        assert_eq!(is_valid_config(21037, &vec![9, 7, 18, 13]), false);
    }

    #[test]
    fn test_generate_permutations() {
        let permutations = generate_permutations(3);
        assert_eq!(permutations.len(), 4);
        assert_eq!(permutations[0], vec!['+', '+']);
        assert_eq!(permutations[1], vec!['*', '+']);
        assert_eq!(permutations[2], vec!['+', '*']);
        assert_eq!(permutations[3], vec!['*', '*']);
        let permutations = generate_permutations(4);
        assert_eq!(permutations.len(), 8);
        assert_eq!(permutations[0], vec!['+', '+', '+']);
        assert_eq!(permutations[1], vec!['*', '+', '+']);
        assert_eq!(permutations[2], vec!['+', '*', '+']);
        assert_eq!(permutations[3], vec!['*', '*', '+']);
        assert_eq!(permutations[4], vec!['+', '+', '*']);
        assert_eq!(permutations[5], vec!['*', '+', '*']);
        assert_eq!(permutations[6], vec!['+', '*', '*']);
        assert_eq!(permutations[7], vec!['*', '*', '*']);
    }

    #[test]
    fn test_get_calibration_and_configs() {
        let line = "190: 10 19";
        let (calib, configs) = get_calibration_and_configs(line).unwrap();
        assert_eq!(calib, 190);
        assert_eq!(configs, vec![10, 19]);

        let line = "3267: 81 40 27";
        let (calib, configs) = get_calibration_and_configs(line).unwrap();
        assert_eq!(calib, 3267);
        assert_eq!(configs, vec![81, 40, 27]);
    }

    #[test]
    fn test_get_total_valid_calibrations() {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        let total = get_total_valid_calibrations(input);
        assert_eq!(total, 3749);
    }
}
