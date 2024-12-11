mod data_types;
mod parser;
use crate::parser::parser::*;


fn main() {
    let lines = parse_file("testing_resources/cmaj_scale_quarternotes.inst".to_string()).unwrap();
    println!("{:?}", lines);
}

