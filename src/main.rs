use std::fs::{File, read_to_string};
use std::io::prelude::*;

const STAFF_SIZE: usize = 11;

fn main() {
    println!("Hello, world!");
}

fn get_file(file_path: String) -> Result<Vec<String>, &'static str> {
    if let Ok(mut file) = File::open(file_path) {
        let mut file_contents = String::new();

        if let Ok(_) = file.read_to_string(&mut file_contents) {
            return Ok(file_contents.lines().map(|str| str.to_string()).collect());
        }

        return Err("File could not be read to string");
    }

    Err("File could not be opened")
}
