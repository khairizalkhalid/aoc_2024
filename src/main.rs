use std::fs::File;
use std::i32;
use std::io::{self, Read};
use std::time::Instant;

fn read_file() -> io::Result<String> {
    let mut file = File::open("./input/day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn split_contents_into_two_vectors(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in contents.lines() {
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

// My own implementation to buble sort. Improved sorting should be using the rust default sort.
#[allow(dead_code)]
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
    vec
}

fn calculate_distance_of_two_vectors(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut distance_vec = Vec::new();
    if vec1.len() != vec2.len() {
        return distance_vec;
    }
    let max = vec1.len();
    for i in 0..max {
        let distance = (vec1[i] - vec2[i]).abs();
        distance_vec.push(distance);
    }
    return distance_vec;
}

fn sum_of_vector(vec: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    let max = vec.len();
    for i in 0..max {
        total += vec[i]
    }
    total
}

fn main() {
    println!("Hello, world!");

    match read_file() {
        Ok(contents) => {
            let start = Instant::now();

            let (mut vec1, mut vec2) = split_contents_into_two_vectors(&contents);
            vec1.sort();
            vec2.sort();
            let distance_vec = calculate_distance_of_two_vectors(vec1, vec2);
            let total_distance = sum_of_vector(distance_vec);
            println!("{}", total_distance);

            let duration = start.elapsed();
            println!("Time elapsed: {:?}", duration);
        }
        Err(e) => println!("Error: {}", e),
    }
}
