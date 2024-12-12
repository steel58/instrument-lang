pub mod parser {
use std::{fs::File, isize, usize};
use regex::Regex;
use std::io::prelude::*;
use crate::data_types::dt::*;

pub fn parse_file(filepath: String) -> Result<Vec<Line>, ParsingError> {
    let contents = get_file(filepath).unwrap();
    let raw_lines = get_raw_lines(contents);
    get_tokenized_lines(raw_lines)
}

fn get_file(file_path: String) -> Result<Vec<String>, ParsingError> {
    if let Ok(mut file) = File::open(file_path.clone()) {
        let mut file_contents = String::new();

        if let Ok(_) = file.read_to_string(&mut file_contents) {
            return Ok(file_contents.lines().map(|str| str.to_string()).collect());
        }

    }

    Err(ParsingError::FailedFileRead(file_path))
}

/*
 * This function takes the lines of code found in filepath and chunks them into
 * lines of music.
 */
fn get_raw_lines(code_lines: Vec<String>) -> Vec<Vec<String>> {
    let mut music_lines: Vec<Vec<String>> = Vec::new();
    let re = Regex::new(r"^=+").unwrap();

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
        if line.len() == 0 { continue; }
        let found_clef = match get_clef_type(line, line_number) {
            Ok(staff) => staff,
            Err(e) => return Err(e),
        };

        let center_index = get_center_line(line, &found_clef);

        let measures = match get_measures(line, measure_count, center_index, &found_clef) {
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
fn get_measures(line: &Vec<String>, measure_count: usize, center: usize, clef: &StaffType) 
    -> Result<Vec<Bar>, ParsingError> {
        let new_measures = line.first().unwrap().match_indices('l').collect::<Vec<_>>().len();
        let mut measures: Vec<Vec<String>> = vec![Vec::new(); new_measures];
        for l in line.iter() {
            for (i, m) in l.split("l").enumerate() {
                if i == 0 { continue; }
                measures[i-1].push(m.to_string());
            }
        }

        let bars: Result<Vec<Bar>, ParsingError> = measures.iter().enumerate()
            .map(|(i, measure)| tokenize_bars(measure, measure_count + i, center, clef))
            .collect();

        bars
    }

fn find_tail(line: &String, head_idx: usize) -> isize {
    let mut tail_side = 0;
    if let Some(tail) = line.chars().nth(head_idx - 1) {
            if tail == '|' {
                tail_side = -1;
            }
    }

    if let Some(tail) = line.chars().nth(head_idx + 1) {
            if tail == '|' {
                tail_side = 1;
            }
    }

    tail_side
}

fn tokenize_bars(measure: &Vec<String>, measure_count: usize, center: usize, clef: &StaffType) 
    -> Result<Bar, ParsingError> {
        //GO THROUGH EACH STEP AND CHECK THE THINGS FOR PLACES THAT ARE GOOD
        let length = measure.first().unwrap().len();
        let mut notes: Vec<Note> = Vec::new();
        let mut durs: Vec<Beats> = Vec::new();

        for line in measure.iter() {
            if line.len() != length && line.len() != 0 {
                return Err(ParsingError::InvalidMeasureLenghts(measure_count));
            }
        }

        for pos in 0..length {
            let mut this_col: Vec<char> = Vec::new();
            for line in measure.iter() {
                if let Some(c) = line.chars().nth(pos) {
                    if c == '@' {
                        find_tail(line, pos);
                    }
                    if c == 'O' {
                        find_tail(line, pos);
                    }
                }
            }

            //This is ugly and only works on quarternotes we shoudl build a function
            //to find the next note and beat so we can make this legible and extensible
            let mut note_height: Option<isize> = None;
            if let Some(n) = note_height {

                let offset: isize = n - center as isize;

                notes.push(calculate_note(offset, clef));
            } else {
                return Err(ParsingError::InvalidNoteDeclaration(measure_count, pos));
            }
        }

        Ok(Bar {
            pitches: notes,
            durations: durs,
            measure_number: measure_count,
        })
    }

/*
 * This will find both the octave and the letter note of a note based on how many
 * lines above or below the middle line.
 * Negative is above the middle line positive is below.
 */
fn calculate_note(offset: isize, clef: &StaffType) -> Note {
    let center_note: Note = match clef {
        StaffType::Bass => BASS_CENTER,
        StaffType::Treble => TREBLE_CENTER,
    };

    let next_index: isize = isize::try_from(NOTE_ORDER.iter()
        .position(|&x| x == center_note.note_name)
        .unwrap()).unwrap() - offset;

    let mut octave_offset: isize = next_index / 7;

    //This modification is to account for the fact that if we drop below 0 we
    //will have 1 fewer octave offset based on the equation.
    //This does not happen if we land squarely on the 0th note ("A")
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

/*
 * This is a helper function to find the lowest possible positive equivalence.
 * Assuming the output is 'z' we could write the output of this as the LaTex
 *                  x \equiv z (mod y)
 */
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
    fn parser_test_calculate_note_positive() {
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
        let c4 = Note {
            accidental: Accidental::Natural,
            note_name: "C",
            octave: 4,
            rest: false,
        };

        let c3 = Note {
            accidental: Accidental::Natural,
            note_name: "C",
            octave: 3,
            rest: false,
        };

        let c2 = Note {
            accidental: Accidental::Natural,
            note_name: "C",
            octave: 2,
            rest: false,
        };

        let c1 = Note {
            accidental: Accidental::Natural,
            note_name: "C",
            octave: 1,
            rest: false,
        };

        assert_eq!(a5, calculate_note(1, &StaffType::Treble));
        assert_eq!(f4, calculate_note(3, &StaffType::Treble));
        assert_eq!(c4, calculate_note(6, &StaffType::Treble));
        assert_eq!(a4, calculate_note(8, &StaffType::Treble));
        assert_eq!(a3, calculate_note(15, &StaffType::Treble));

        assert_eq!(c3, calculate_note(1, &StaffType::Bass));
        assert_eq!(a3, calculate_note(3, &StaffType::Bass));
        assert_eq!(c2, calculate_note(8, &StaffType::Bass));
        assert_eq!(c1, calculate_note(15, &StaffType::Bass));
    }

    #[test]
    fn test_parser_calculate_note_negative() {
        let d6 = Note {
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

        let c4 = Note {
            accidental: Accidental::Natural,
            note_name: "C",
            octave: 4,
            rest: false,
        };

        let e3 = Note {
            accidental: Accidental::Natural,
            note_name: "E",
            octave: 3,
            rest: false,
        };

        let f4 = Note {
            accidental: Accidental::Natural,
            note_name: "F",
            octave: 4,
            rest: false,
        };

        assert_eq!(d6, calculate_note(-9, &StaffType::Treble));
        assert_eq!(a6, calculate_note(-6, &StaffType::Treble));
        assert_eq!(c5, calculate_note(-1, &StaffType::Treble));

        assert_eq!(f4, calculate_note(-9, &StaffType::Bass));
        assert_eq!(c4, calculate_note(-6, &StaffType::Bass));
        assert_eq!(e3, calculate_note(-1, &StaffType::Bass));
    }

    #[test]
    fn test_parser_calculate_note() {
        let b5 = Note {
            accidental: Accidental::Natural,
            note_name: "B",
            octave: 5,
            rest: false,
        };
        let d3 = Note {
            accidental: Accidental::Natural,
            note_name: "D",
            octave: 3,
            rest: false,
        };

        assert_eq!(b5, calculate_note(0, &StaffType::Treble));
        assert_eq!(d3, calculate_note(0, &StaffType::Bass));
    }

    #[test]
    fn test_parser_cmaj_scale() {
        let c4 = Note { 
            accidental: Accidental::Natural,
            note_name: "C",
            octave: 4,
            rest: false,
        };
        let d4 = Note { 
            accidental: Accidental::Natural,
            note_name: "D",
            octave: 4,
            rest: false,
        };
        let e4 = Note { 
            accidental: Accidental::Natural,
            note_name: "E",
            octave: 4,
            rest: false,
        };
        let f4 = Note { 
            accidental: Accidental::Natural,
            note_name: "F",
            octave: 4,
            rest: false,
        };
        let g4 = Note { 
            accidental: Accidental::Natural,
            note_name: "G",
            octave: 4,
            rest: false,
        };
        let a5 = Note { 
            accidental: Accidental::Natural,
            note_name: "A",
            octave: 5,
            rest: false,
        };
        let b5 = Note { 
            accidental: Accidental::Natural,
            note_name: "B",
            octave: 5,
            rest: false,
        };
        let c5 = Note { 
            accidental: Accidental::Natural,
            note_name: "C",
            octave: 5,
            rest: false,
        };

        let contents = get_file("testing_resources/cmaj_scale_quarternotes.inst".to_string()).unwrap();
        let raw_lines = get_raw_lines(contents);
        let lines = get_tokenized_lines(raw_lines).unwrap();

        let mut expected: Vec<Line> = Vec::new();
        let mut expected_bars: Vec<Bar> = Vec::new();
        expected_bars.push(Bar {
            pitches: vec![c4.clone(), d4.clone(), e4.clone(), f4.clone()],
            durations: vec![Beats::Quarter; 4],
            measure_number: 1,
        });
        expected_bars.push(Bar {
            pitches: vec![g4.clone(), a5.clone(), b5.clone(), c5.clone()],
            durations: vec![Beats::Quarter; 4],
            measure_number: 2,
        });
        expected_bars.push(Bar {
            pitches: vec![c5, b5, a5, g4],
            durations: vec![Beats::Quarter; 4],
            measure_number: 3,
        });
        expected_bars.push(Bar {
            pitches: vec![f4, e4, d4, c4],
            durations: vec![Beats::Quarter; 4],
            measure_number: 4,
        });
        expected_bars.push(Bar {
            pitches: Vec::new(),
            durations: Vec::new(),
            measure_number: 5,
        });
        expected_bars.push(Bar {
            pitches: Vec::new(),
            durations: Vec::new(),
            measure_number: 6,
        });
        let expected_line = Line {
            clef_type: StaffType::Treble,
            line_height: 12,
            center_line: 5,
            contents: expected_bars,
        };

        expected.push(expected_line);

        assert_eq!(expected, lines);
    }
}
}
