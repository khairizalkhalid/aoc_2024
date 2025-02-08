use crate::utils;

const TARGET_WORD: &str = "XMAS";

fn count_word(target_word: &str, input_vec: &Vec<Vec<char>>) -> i32 {
    let target_chars: Vec<char> = target_word.chars().collect();

    input_vec
        .iter()
        .enumerate()
        .map(|(y_i, y)| {
            y.iter()
                .enumerate()
                .map(|(x_i, _)| count_match_chars(&target_chars, (x_i, y_i), input_vec))
                .sum::<i32>()
        })
        .sum()
}

fn count_match_chars(
    target_chars: &Vec<char>,
    root_index: (usize, usize),
    input_vec: &Vec<Vec<char>>,
) -> i32 {
    // loop to check all of the direction
    let mut count = 0;
    let (x_i, y_i) = root_index;
    let directions = [
        (1, 0),   // east
        (1, 1),   // southeast
        (0, 1),   // south
        (-1, 1),  // southwest
        (-1, 0),  // west
        (-1, -1), // northwest
        (0, -1),  // north
        (1, -1),  // northeast
    ];

    for (dx, dy) in directions.iter() {
        let mut match_found = true;
        for (i, &target_char) in target_chars.iter().enumerate() {
            let new_x = x_i as isize + i as isize * dx;
            let new_y = y_i as isize + i as isize * dy;
            if new_x < 0
                || new_y < 0
                || new_x >= input_vec[0].len() as isize
                || new_y >= input_vec.len() as isize
            {
                match_found = false;
                break;
            }
            if input_vec[new_y as usize][new_x as usize] != target_char {
                match_found = false;
                break;
            }
        }
        if match_found {
            count += 1;
        }
    }

    count
}

pub fn test_run() {
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

pub fn run() {
    match utils::file_reader::read_file("day4.txt") {
        Ok(contents) => println!(
            "{:?}",
            count_word(
                TARGET_WORD,
                &contents.lines().map(|l| l.chars().collect()).collect()
            )
        ),
        Err(e) => println!("Err: {}", e),
    }
}
