use std::fs::File;
use std::io::{self, Read};

fn read_file() -> io::Result<String> {
    let mut file = File::open("./input/day1.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let max = vec.len();
    let mut temp: i32;
    let mut swapped: bool;
    for i in 0..max - 1 {
        swapped = false;
        for j in 0..max - i - 1 {
            if vec[j] > vec[j + 1] {
                temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    return vec;
}

fn main() {
    println!("Hello, world!");

    let contents = read_file();

    match contents {
        Ok(contents) => {
            let lines: Vec<&str> = contents.lines().collect();
            println!("{:?}", lines);
        }
        Err(e) => println!("Error: {}", e),
    }
}
