use std::{fs::File, iter::Enumerate, str::Lines};
use regex::Regex;
use std::io::prelude::*;
use thiserror::Error;

const BASS_CENTER: &str = "D3";
const TREBLE_CENTER: &str = "B5";

#[derive(Error, Debug)]
enum ParsingError {
    #[error("Invalid Staff Identifier at line {0}")]
    InvalidStaffDeclaration(usize),
}

enum StaffType {
    Treble,
    Bass,
}

enum Beats {
    ThirtySecond,
    Sixteenth,
    Eighth,
    Quarter,
    Half,
    Whole,
    DottedThirtySecond,
    DottedSixteenth,
    DottedEighth,
    DottedQuarter,
    DottedHalf,
    DottedWhole,
    EighthTriplet,
    QuarterTriplet,
    HalfTriplet,
}

enum Accidental {
    Sharp,
    Flat,
    Natural,
}

struct Line {
    clef_type: StaffType,
    line_height: usize,
    center_line: usize,
    contents: Vec<Bar>,
}

struct Bar {
    pitches: Vec<Note>,
    durations: Vec<Beats>,
    measure_number: usize,
}

struct Note {
    acidental: Accidental,
    note_name: String,
    octave: usize,
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
fn get_raw_lines(code_lines: Vec<String>) -> Vec<Vec<String>> {
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


fn get_tokenized_lines(lines: Vec<Vec<String>>) -> Result<Vec<Line>, ParsingError> {
    for (line_number, line) in lines.iter().enumerate() {
        let clef_type = match get_clef_type(line, line_number) {
            Ok(staff) => staff,
            Err(e) => return Err(e),
        };

        let center_index = get_center_line(line, clef_type);
        //Break into bars please
    }

    Ok(Vec::new())
}


/*
 * This function takes a group of Strings that is one line of music (aka several
 * bars)
 */
fn get_clef_type(line: &Vec<String>, line_number: usize) -> Result<StaffType, ParsingError> {
    // Get find the top regex and then make sure that they are all in order
    todo!()
}

/*
 * This function takes a group of strings that is one line of music and returns
 * the index of the center line (the note B4 for treble and D3 for bass)
 */
fn get_center_line(line: &Vec<String>, clef_type: StaffType) -> usize {
    let center_finder = match clef_type {
        StaffType::Bass => Regex::new(r"^\ {3}\/\ {4}l"),
        StaffType::Treble => Regex::new(r"^\ \/\ \|\ {3}l"),
    }.unwrap();

    let mut center_index = 0;
    for (idx, row) in line.iter().enumerate() {
        if center_finder.is_match(row) {
            center_index = idx;
            break;
        }
    }

    center_index
}
