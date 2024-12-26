use std::fs::File;
use std::i32;
use std::io::{self, Read};

fn read_file() -> io::Result<String> {
    let mut file = File::open("./input/day1.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn split_contents_into_two_vectors(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<&str> = contents.lines().collect();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines {
        let items: Vec<&str> = line.split_whitespace().collect();
        if items.len() == 2 {
            if let Ok(item1) = items[0].parse::<i32>() {
                vec1.push(item1);
            }
            if let Ok(item2) = items[1].parse::<i32>() {
                vec2.push(item2);
            }
        }
    }

    (vec1, vec2)
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
            let (vec1, vec2) = split_contents_into_two_vectors(&contents);
            println!("vec1, {:?}", vec1);
            println!("vec2, {:?}", vec2);
            let sort_vec1 = bubble_sort(vec1);
            let sort_vec2 = bubble_sort(vec2);
            println!("sort_vec1, {:?}", sort_vec1);
            println!("sort_vec2, {:?}", sort_vec2);
        }
        Err(e) => println!("Error: {}", e),
    }
}
