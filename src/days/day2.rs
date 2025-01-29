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

fn safety_check(readings: Vec<i32>) -> bool {
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
}
