use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()?;
    Ok(lines)
}
