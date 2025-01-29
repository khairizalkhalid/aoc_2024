use std::fs::File;
use std::io::{self, Read};

pub fn read_file(file_name: &str) -> io::Result<String> {
    let path = format!("./input/{}", file_name);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
