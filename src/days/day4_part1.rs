const TARGET_WORD: &str = "XMAS";

fn count_word(target_word: &str, input_vec: &Vec<Vec<char>>) -> i32 {
    let target_chars: Vec<char> = target_word.chars().collect();

    let count_vec: Vec<i32> = input_vec
        .iter()
        .map(|y| {
            y.iter()
                .map(|x| count_match_chars(target_word, input_vec))
                .sum()
        })
        .collect();
    count_vec.iter().sum()
}

fn count_match_chars(target_word: &str, input_vec: &Vec<Vec<char>>) -> i32 {
    1
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

    let mut str_coordinate: Vec<Vec<char>> =
        test_str.lines().map(|l| l.chars().collect()).collect();

    println!("test {:?}", str_coordinate);

    println!(
        "{} word count: {}",
        TARGET_WORD,
        count_word(TARGET_WORD, &str_coordinate)
    );
}
