// test case:
//
// 190: 10 19
// 3267: 81 40 27
// 83: 17 5
// 156: 15 6
// 7290: 6 8 6 15
// 161011: 16 10 13
// 192: 17 8 14
// 21037: 9 7 18 13
// 292: 11 6 16 20
//
// task: in each line, find which one is correct if "x : a b c" where x must be equal a, b, c using
// operators "+" and "*"
// then add up all the one that is correct.

fn is_valid_config(calib: i32, configs: Vec<i32>) -> bool {
    // in configs, calculate using 2 operators (+ and *)
    // if either is exceeding the calib, return false
    // end loop, total is equal calib, return true else false

    let mut cfg_ops_result = configs[0];

    for &val in configs.iter().skip(1) {
        cfg_ops_result *= val;
    }

    cfg_ops_result == calib
}

// function to write plus or multiply permutations based on the number of configs minus 1
// e.g. if configs = [1, 2, 3], then we have 2 operators to choose from
// we can have 2^2 = 4 permutations
// we can use a bitmask to generate the permutations
// ie: 0: [ "+", "+" ]
// 1: [ "+", "*" ]
// 2: [ "*", "+" ]
// 3: [ "*", "*" ]
fn generate_permutations(num_configs: usize) -> Vec<Vec<&'static str>> {
    let mut permutations = Vec::new();
    let total_permutations = 1 << (num_configs - 1); // 2^(num_configs - 1)

    for i in 0..total_permutations {
        let mut ops = Vec::new();
        for j in 0..(num_configs - 1) {
            if (i & (1 << j)) != 0 {
                ops.push("*");
            } else {
                ops.push("+");
            }
        }
        permutations.push(ops);
    }

    permutations
}

#[cfg(test)]
mod test {
    use crate::days::day7_part1::{generate_permutations, is_valid_config};


    #[test]
    fn test_is_valid_config() {
        let calib_1 = 190;
        let configs_1 = vec![10, 19];
        assert_eq!(is_valid_config(calib_1, configs_1), true);

        let calib_2 = 3267;
        let configs_2 = vec![81, 40, 27];
        assert_eq!(is_valid_config(calib_2, configs_2), true);
    }

    fn test_generate_permutations() {
        let permutations = generate_permutations(3);
        assert_eq!(permutations.len(), 4);
        assert_eq!(permutations[0], vec!["+", "+"]);
        assert_eq!(permutations[1], vec!["*", "+"]);
        assert_eq!(permutations[2], vec!["+", "*"]);
        assert_eq!(permutations[3], vec!["*", "*"]);
        let permutations = generate_permutations(4);
        assert_eq!(permutations.len(), 8);
        assert_eq!(permutations[0], vec!["+", "+", "+"]);
        assert_eq!(permutations[1], vec!["*", "+", "+"]);
        assert_eq!(permutations[2], vec!["+", "*", "+"]);
        assert_eq!(permutations[3], vec!["*", "*", "+"]);
        assert_eq!(permutations[4], vec!["+", "+", "*"]);
        assert_eq!(permutations[5], vec!["*", "+", "*"]);
        assert_eq!(permutations[6], vec!["+", "*", "*"]);
        assert_eq!(permutations[7], vec!["*", "*", "*"]);
    }
}
