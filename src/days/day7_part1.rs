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

#[cfg(test)]
mod test {
    use crate::days::day7_part1::is_valid_config;

    #[test]
    fn test_is_valid_config() {
        let calib_1 = 190;
        let configs_1 = vec![10, 19];
        assert_eq!(is_valid_config(calib_1, configs_1), true);

        let calib_2 = 3267;
        let configs_2 = vec![81, 40, 27];
        assert_eq!(is_valid_config(calib_2, configs_2), true);
    }
}
