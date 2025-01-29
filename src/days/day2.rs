const SAFE: bool = true;
const POSITIVE: bool = true;

fn safety_check(vec_z: Vec<i32>) -> bool {
    let max: usize = (vec_z.len() - 1).try_into().unwrap();
    let mut direction: bool = POSITIVE;
    let mut safety: bool = SAFE;
    for i in 0..max {
        let z = vec_z[i];
        let next_z = vec_z[i + 1];

        if i == 0 {
            if z > next_z {
                direction = POSITIVE
            } else {
                direction = !POSITIVE
            }
        } else if (direction == !POSITIVE && z > next_z) || (direction == POSITIVE && next_z > z) {
            safety = !SAFE;
            break;
        }

        let safe_range: i32 = z - next_z;
        let abs_range = safe_range.abs();
        if abs_range > 3 || abs_range == 0 {
            safety = !SAFE;
            break;
        }
    }
    safety
}

//test result
//7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
//1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
//9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
//1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
//8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
//1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
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
    let test_case = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];

    let result: Vec<bool> = test_case
        .iter()
        .map(|vec_z| safety_check(vec_z.to_vec()))
        .collect();
    println!("{:?}", result);
    println!("{:?}", result.iter().filter(|&&r| r).count());
}
