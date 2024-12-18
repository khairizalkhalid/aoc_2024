use std::fs::File;
use std::io::{self, Read};

fn read_file() -> io::Result<String> {
    let mut file = File::open("./input/day1.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn main() {
    println!("Hello, world!");

    let contents = read_file();

    match contents {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error: {}", e),
    }
}
