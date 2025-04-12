use crate::utils;

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

    let backslash_match = is_half_cross(&backslash_directions, target_chars, (x_i, y_i), input_vec);
    let slash_match = is_half_cross(&slash_directions, target_chars, (x_i, y_i), input_vec);

    backslash_match && slash_match
}

fn is_half_cross(
    directions: &[(isize, isize)],
    target_chars: &[char],
    (x_i, y_i): (usize, usize),
    input_vec: &[Vec<char>],
) -> bool {
    let mut half_cross = false;
    for (dx, dy) in directions.iter() {
        for (i, &target_char) in target_chars.iter().enumerate() {
            let new_x = x_i as isize - (1 * dx) + i as isize * dx;
            let new_y = y_i as isize - (1 * dy) + i as isize * dy;
            if new_x < 0
                || new_y < 0
                || new_x >= input_vec[0].len() as isize
                || new_y >= input_vec.len() as isize
            {
                half_cross = false;
                break;
            }
            if input_vec[new_y as usize][new_x as usize] != target_char {
                half_cross = false;
                break;
            }
            half_cross = true;
        }
        if half_cross {
            break;
        }
    }
    half_cross
}

#[allow(dead_code)]
pub fn test() {
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
                &contents
                    .lines()
                    .map(|l| l.chars().collect())
                    .collect::<Vec<_>>()
            )
        ),
        Err(e) => println!("Err: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_word() {
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
        let str_coordinate: Vec<Vec<char>> =
            test_str.lines().map(|l| l.chars().collect()).collect();

        assert_eq!(count_word(TARGET_WORD, &str_coordinate), 9);
    }

    #[test]
    fn test_is_half_cross() {
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
        let str_coordinate: Vec<Vec<char>> =
            test_str.lines().map(|l| l.chars().collect()).collect();

        assert_eq!(
            is_half_cross(
                &[(1, 1), (-1, -1)],
                &TARGET_WORD.chars().collect::<Vec<_>>(),
                (2, 4),
                &str_coordinate
            ),
            true
        );
    }
}
