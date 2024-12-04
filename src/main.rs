use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    print!("{}", contents);
    Ok(())
}
