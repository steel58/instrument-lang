use std::fs::File;
use regex::Regex;
use std::io::prelude::*;

const BASS_CENTER: &str = "D3";
const TREBLE_CENTER: &str = "B5";

enum StaffType {
    Treble,
    Bass,
}

struct Line {
    clef_type: StaffType,
    line_height: usize,
    center_line: usize,
    contents: Vec<String>,
}

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

/*
 * This function takes the lines of code found in filepath and chunks them into
 * lines of music.
 */
fn get_lines(code_lines: Vec<String>) -> Vec<Vec<String>> {
    let mut music_lines: Vec<Vec<String>> = Vec::new();
    let re = Regex::new(r"^=+$").unwrap();

    let mut line_builder: Vec<String> = Vec::new();
    for row in code_lines.iter() {
        if re.is_match(row) {
            music_lines.push(line_builder);
            line_builder = Vec::new();
            continue;
        } 

        line_builder.push(row.clone());
    }
    music_lines.remove(0); //Take out the empty vec that got placed there at file
                           //instantiation
    music_lines
}

/*
 * This function takes a group of Strings that is one line of music (aka several
 * bars)
 */
fn get_clef_type(line: Vec<String>) -> StaffType {
    todo!()
}

/*
 * This function takes a group of strings that is one line of music and returns
 * the index of the center line (the note B4 for treble and D3 for bass)
 */
fn get_center_line(line: Vec<String>, clef_type: StaffType) -> Result<usize, &'static str> {
    let center_finder = match clef_type {
        StaffType::Bass => Regex::new(r"^\ {3}\/\ {4}l"),
        StaffType::Treble => Regex::new(r"^\ \/\ \|\ {3}l"),
    }.unwrap();

    for (idx, row) in line.iter().enumerate() {
        if center_finder.is_match(row) {
            return Ok(idx);
        }
    }

    Err("Center not found, invalid line was passed")
}
