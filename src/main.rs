use std::{fs::File, isize, usize};
use regex::{Regex};
use std::io::prelude::*;
mod data_types;
use crate::data_types::dt::*;


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
    let mut measure_count: usize = 1;
    let mut result: Vec<Line> = Vec::new();
    for (line_number, line) in lines.iter().enumerate() {
        let found_clef = match get_clef_type(line, line_number) {
            Ok(staff) => staff,
            Err(e) => return Err(e),
        };

        let center_index = get_center_line(line, &found_clef);

        let measures = match get_measures(line, measure_count, &found_clef) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
        
        measure_count += measures.len();

        result.push( Line {
                clef_type: found_clef,
                line_height: line.len(),
                center_line: center_index,
                contents: measures,
            });
    }

    Ok(result)
}

/*
 * This function breaks a line into measures else it bricks the shit.
 *
 */
fn get_measures(line: &Vec<String>, measure_count: usize, clef: &StaffType) -> Result<Vec<Bar>, ParsingError> {
    let new_measures = line.first().unwrap().match_indices('l').collect::<Vec<_>>().len();
    let mut measures: Vec<Vec<String>> = vec![Vec::new(); new_measures];
    for l in line.iter() {
        for (i, m) in l.split("l").enumerate() {
            measures[i].push(m.to_string());
        }
    }

    let bars: Result<Vec<Bar>, ParsingError> = measures.iter().enumerate()
        .map(|(i, measure)| tokenize_bars(measure, measure_count + i, clef))
        .collect();

    bars
}

fn tokenize_bars(measure: &Vec<String>, measure_count: usize, clef: &StaffType) -> Result<Bar, ParsingError> {
    //GO THROUGH EACH STEP AND CHECK THE THINGS FOR PLACES THAT ARE GOOD
    let center = get_center_line(measure, clef);
    let length = measure.first().unwrap().len();
    let mut notes: Vec<Note> = Vec::new();
    let mut durs: Vec<Beats> = Vec::new();

    for line in measure.iter() {
        if line.len() != length {
            return Err(ParsingError::InvalidMeasureLenghts(measure_count));
        }
    }

    for pos in 0..length {
        let mut this_col: Vec<char> = Vec::new();
        for line in measure.iter() {
            this_col.push(line.chars().nth(pos).unwrap());
        }

        let mut note_height: Option<usize> = None;
        if this_col.contains(&'|') {
            let first = this_col.iter().position(|r| *r == '|').unwrap();
            let last = this_col.len() - 1 - this_col.iter()
                .rev().position(|r| *r == '|').unwrap();
            if let Some(head) = measure.iter().nth(first).unwrap().chars().nth(pos -1) {
                if head == '@' {
                    note_height = Some(first);
                }
            }

            if let Some(head) = measure.iter().nth(first).unwrap().chars().nth(pos + 1) {
                if head == '@' {
                    note_height = Some(first);
                }
            }

            if let Some(head) = measure.iter().nth(last).unwrap().chars().nth(pos + 1) {
                if head == '@' {
                    note_height = Some(last);
                }
            }

            if let Some(head) = measure.iter().nth(last).unwrap().chars().nth(pos - 1) {
                if head == '@' {
                    note_height = Some(last);
                }
            }

            durs.push(Beats::Quarter);
            if let Some(n) = note_height {
                let offset = n - center;
                
            } else {

            }

        }
    }

    Ok(Bar {
        pitches: notes,
        durations: durs,
        measure_number: measure_count,
    })
}

fn calculate_note(offset: isize, clef: &StaffType) -> Note {
    let center_note: Note = match clef {
        StaffType::Bass => BASS_CENTER,
        StaffType::Treble => TREBLE_CENTER,
    };

    let next_index: isize = isize::try_from(NOTE_ORDER.iter()
        .position(|&x| x == center_note.note_name)
        .unwrap()).unwrap() - offset;

    let mut octave_offset: isize = next_index / 7;

    if next_index < 0 && modulo(next_index, 7) != 0{
        octave_offset -= 1;
    }


    Note { 
        accidental: Accidental::Natural,
        note_name: NOTE_ORDER[modulo(next_index, 7)],
        octave: (center_note.octave as isize + octave_offset) as usize, 
        rest: false,
    }
}

fn modulo(x: isize, y: usize) -> usize {
    if x >= 0 {
        x as usize % y
    } else {
        let mut equiv = x;
        while equiv < 0 {
            equiv += y as isize;
        }

        equiv as usize % y
    }
}


/*
 * This function takes a group of Strings that is one line of music (aka several
 * bars)
 */
fn get_clef_type(line: &Vec<String>, line_number: usize) -> Result<StaffType, ParsingError> {
    let mut found = false;
    let mut found_type = StaffType::Treble;
    let mut strip_counter = 0;
    // Get find the top regex and then make sure that they are all in order
    let mut treble_regex: Vec<Regex> = Vec::new();
    treble_regex.push(Regex::new(r"^ {3}\/\\").unwrap());
    treble_regex.push(Regex::new(r"^ {3}\| \\ l").unwrap());
    treble_regex.push(Regex::new(r"^ {3}\| \/ l").unwrap());
    treble_regex.push(Regex::new(r"^ {3}\|\/  l").unwrap());
    treble_regex.push(Regex::new(r"^  \/\| {3}l").unwrap());
    treble_regex.push(Regex::new(r"^ \/ \| {3}l").unwrap());
    treble_regex.push(Regex::new(r"^\|  \|\\  l").unwrap());
    treble_regex.push(Regex::new(r"^ \\ \| \| l").unwrap());
    treble_regex.push(Regex::new(r"^  \\\|\/  l").unwrap());
    treble_regex.push(Regex::new(r"^\/@ \| {3}l").unwrap());
    treble_regex.push(Regex::new(r"^\\_\/").unwrap());

    let mut bass_regex: Vec<Regex> = Vec::new();
    bass_regex.push(Regex::new(r"^ __").unwrap());
    bass_regex.push(Regex::new(r"^").unwrap());
    bass_regex.push(Regex::new(r"^\/  \\ {4}l").unwrap());
    bass_regex.push(Regex::new(r"^\| {3}\\ @ l").unwrap());
    bass_regex.push(Regex::new(r"^\\@@ \| {3}l").unwrap());
    bass_regex.push(Regex::new(r"^ @@ \/ @ l").unwrap());
    bass_regex.push(Regex::new(r"^ {3}\/ {4}l").unwrap());
    bass_regex.push(Regex::new(r"^ {2}\/ {5}l").unwrap());
    bass_regex.push(Regex::new(r"^ \/ {6}l").unwrap());
    bass_regex.push(Regex::new(r"^\/ {7}l").unwrap());
    bass_regex.push(Regex::new(r"^ {8}l").unwrap());

    for strip in line.iter() {
        if !found {
            if bass_regex.first().unwrap().is_match(strip) {
                found_type = StaffType::Bass;
                found = true;
                strip_counter = 1;
            }
            else if treble_regex.first().unwrap().is_match(strip) {
                found_type = StaffType::Treble;
                found = true;
                strip_counter = 1;
            }
            
            continue;
        }

        let correct = match found_type {
            StaffType::Bass => bass_regex.iter()
                .nth(strip_counter)
                .unwrap()
                .is_match(strip),
            StaffType::Treble => treble_regex.iter()
                .nth(strip_counter)
                .unwrap()
                .is_match(strip),
        };

        if !correct {
            return Err(ParsingError::InvalidStaffDeclaration(line_number))
        }

        if strip_counter == 10 {
            return Ok(found_type)
        }

        strip_counter += 1;
    }

    Err(ParsingError::InvalidStaffDeclaration(line_number))
}

/*
 * This function takes a group of strings that is one line of music and returns
 * the index of the center line (the note B4 for treble and D3 for bass)
 */
fn get_center_line(line: &Vec<String>, clef_type: &StaffType) -> usize {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_note_positive() {
        let a4 = Note {
            accidental: Accidental::Natural,
            note_name: "A",
            octave: 4,
            rest: false,
        };

        let a3 = Note {
            accidental: Accidental::Natural,
            note_name: "A",
            octave: 3,
            rest: false,
        };

        let a5 = Note {
            accidental: Accidental::Natural,
            note_name: "A",
            octave: 5,
            rest: false,
        };

        let f4 = Note {
            accidental: Accidental::Natural,
            note_name: "F",
            octave: 4,
            rest: false,
        };

        assert_eq!(a5, calculate_note(1, &StaffType::Treble));
        assert_eq!(f4, calculate_note(3, &StaffType::Treble));
        assert_eq!(a4, calculate_note(8, &StaffType::Treble));
        assert_eq!(a3, calculate_note(15, &StaffType::Treble));
    }

    #[test]
    fn test_calculate_note_negative() {
        let d3 = Note {
            accidental: Accidental::Natural,
            note_name: "D",
            octave: 6,
            rest: false,
        };

        let a6 = Note {
            accidental: Accidental::Natural,
            note_name: "A",
            octave: 6,
            rest: false,
        };
    
        let c5 = Note {
            accidental: Accidental::Natural,
            note_name: "C",
            octave: 5,
            rest: false,
        };

        assert_eq!(d3, calculate_note(-9, &StaffType::Treble));
        assert_eq!(a6, calculate_note(-6, &StaffType::Treble));
        assert_eq!(c5, calculate_note(-1, &StaffType::Treble));
    }

    #[test]
    fn test_calculate_note() {
        let b5 = Note {
            accidental: Accidental::Natural,
            note_name: "B",
            octave: 5,
            rest: false,
        };

        assert_eq!(b5, calculate_note(0, &StaffType::Treble));
    }
}

