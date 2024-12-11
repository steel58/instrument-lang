pub mod dt {

    use thiserror::Error;

    pub const BASS_CENTER: Note = Note {
        accidental: Accidental::Natural,
        note_name: "D",
        octave: 3,
        rest: false,
    };

    pub const TREBLE_CENTER: Note = Note {
        accidental: Accidental::Natural,
        note_name: "B",
        octave: 5,
        rest: false,
    };

    pub const NOTE_ORDER: [&str; 7] = ["A", "B", "C", "D", "E", "F", "G"];

#[derive(Error, Debug)]
    pub enum ParsingError {
        #[error("Invalid Staff Identifier at line {0}")]
        InvalidStaffDeclaration(usize),
        #[error("Invalid Measure Lengths in measure {0}")]
        InvalidMeasureLenghts(usize),
        #[error("Invalid Note Declatation in measure {0} at {1}")]
        InvalidNoteDeclaration(usize, usize),
        #[error(r#"Failed to read file: "{0}""#)]
        FailedFileRead(String),
    }

    #[derive(Debug)]
    pub enum StaffType {
        Treble,
        Bass,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Beats {
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

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Accidental {
        Sharp,
        Flat,
        Natural,
    }

    #[derive(Debug)]
    pub struct Line {
        pub clef_type: StaffType,
        pub line_height: usize,
        pub center_line: usize,
        pub contents: Vec<Bar>,
    }

    #[derive(Debug)]
    pub struct Bar {
        pub pitches: Vec<Note>,
        pub durations: Vec<Beats>,
        pub measure_number: usize,
    }

    #[derive(Clone, Debug)]
    pub struct Note {
        pub accidental: Accidental,
        pub note_name: &'static str,
        pub octave: usize,
        pub rest: bool,
    }

    impl PartialEq for Note {
        fn eq(&self, other: &Self) -> bool {
            if self.rest && other.rest { return true; }
            if self.rest != other.rest { return false; }

            if self.accidental == other.accidental {
                return self.note_name == other.note_name 
                    && self.octave == other.octave;
            }

            false
        }
    }

    impl PartialEq for Line {
        fn eq(&self, other: &Self) -> bool {
            self.contents == other.contents
        }
    }

    impl PartialEq for Bar {
        fn eq(&self, other: &Self) -> bool {
            self.pitches == other.pitches && self.durations == other.durations
        }
    }
}
