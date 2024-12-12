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

    pub struct TimeSignature {
        pub beats: usize,
        pub beat_size: Beats,
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

    impl Beats {
        pub fn equivalent(&self, other: Beats, num :usize) -> bool {
            match self {
                Self::Eighth => match (other, num) {
                    (Self::ThirtySecond, 4) => true,
                    (Self::Sixteenth, 2) => true,
                    _ => false,
                },
                Self::Quarter => match (other, num) {
                    (Self::EighthTriplet, 3) => true,
                    (Self::Eighth, 2) => true,
                    (Self::Sixteenth, 4) => true,
                    (Self::ThirtySecond, 8) => true,
                    _ => false,
                },
                Self::Half => match (other, num) {
                    (Self::EighthTriplet, 6) => true,
                    (Self::QuarterTriplet, 3) => true,
                    (Self::Quarter, 2) => true,
                    (Self::Eighth, 4) => true,
                    (Self::Sixteenth, 8) => true,
                    (Self::ThirtySecond, 16) => true,
                    _ => false,
                },
                Self::Whole => match (other, num) {
                    (Self::EighthTriplet, 12) => true,
                    (Self::QuarterTriplet, 6) => true,
                    (Self::HalfTriplet, 3) => true,
                    (Self::Half, 2) => true,
                    (Self::Quarter, 4) => true,
                    (Self::Eighth, 8) => true,
                    (Self::Sixteenth, 16) => true,
                    (Self::ThirtySecond, 32) => true,
                    _ => false,
                },
                _ => false,
            }
        }
    }
}
