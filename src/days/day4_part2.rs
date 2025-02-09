const TARGET_WORD: &str = "MAS";

fn count_word(target_word: &str, input_vec: &[Vec<char>]) -> i32 {
    let target_chars: Vec<char> = target_word.chars().collect();

    input_vec
        .iter()
        .enumerate()
        .map(|(y_i, y)| {
            y.iter()
                .enumerate()
                .filter(|(x_i, _)| is_match_x(&target_chars, (*x_i, y_i), input_vec))
                .count() as i32
        })
        .sum()
}

fn is_match_x(target_chars: &[char], root_index: (usize, usize), input_vec: &[Vec<char>]) -> bool {
    let (x_i, y_i) = root_index;
    let backslash_directions = [
        (1, 1),   // southeast
        (-1, -1), // northwest
    ];

    let slash_directions = [
        (-1, 1), // southwest
        (1, -1), // northeast
    ];

    let mut backslash_match = true;
    let mut slash_match = true;

    for (dx, dy) in backslash_directions.iter() {
        for (i, &target_char) in target_chars.iter().enumerate() {
            let new_x = x_i as isize - (1 * dx) + i as isize * dx;
            let new_y = y_i as isize - (1 * dy) + i as isize * dy;
            if new_x < 0
                || new_y < 0
                || new_x >= input_vec[0].len() as isize
                || new_y >= input_vec.len() as isize
            {
                backslash_match = false;
                break;
            }
            if input_vec[new_y as usize][new_x as usize] != target_char {
                backslash_match = false;
                break;
            }
            backslash_match = true;
        }
        if backslash_match {
            break;
        }
    }

    for (dx, dy) in slash_directions.iter() {
        for (i, &target_char) in target_chars.iter().enumerate() {
            let new_x = x_i as isize - (1 * dx) + i as isize * dx;
            let new_y = y_i as isize - (1 * dy) + i as isize * dy;
            if new_x < 0
                || new_y < 0
                || new_x >= input_vec[0].len() as isize
                || new_y >= input_vec.len() as isize
            {
                slash_match = false;
                break;
            }
            if input_vec[new_y as usize][new_x as usize] != target_char {
                slash_match = false;
                break;
            }
            slash_match = true;
        }
        if slash_match {
            break;
        }
    }

    backslash_match && slash_match
}

pub fn run() {
    let test_str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let str_coordinate: Vec<Vec<char>> = test_str.lines().map(|l| l.chars().collect()).collect();

    println!(
        "{} word count: {}",
        TARGET_WORD,
        count_word(TARGET_WORD, &str_coordinate)
    );
}
