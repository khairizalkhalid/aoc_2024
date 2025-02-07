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

    let mut coordinate: Vec<Vec<char>> = test_str.lines().map(|l| l.chars().collect()).collect();

    println!("test {:?}", coordinate)
}
