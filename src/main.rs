use std::fs::File;
use std::io::{self, Read};

fn read_file() -> io::Result<String> {
    let mut file = File::open("./input/day1.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn bubble_sort(mut array_list: [i32; 5]) -> [i32; 5] {
    let max = array_list.len();
    let mut temp: i32;
    let mut swapped: bool;

    for i in 0..max - 1 {
        swapped = false;
        for j in 0..max - i - 1 {
            if array_list[j] > array_list[j + 1] {
                temp = array_list[j];
                array_list[j] = array_list[j + 1];
                array_list[j + 1] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    return array_list;
}

fn main() {
    println!("Hello, world!");

    let contents = read_file();

    match contents {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error: {}", e),
    }

    let an_array = [3, 2, 9, 4, 1];
    let sorted_array = bubble_sort(an_array);
    println!("Bubbly {:?}", sorted_array)
}
