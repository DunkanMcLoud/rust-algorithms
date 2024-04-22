use std::fs::File;
use std::io::{self, BufReader};
use std::io::Read;
use std::usize;

pub const FILE_NOT_FOUND_MSG: &str = "Can't find a file";

pub fn read_as_string(file: File) -> String {
    let mut reader = io::BufReader::new(file);
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Cannot read");
    s
}

pub fn read_numbers_from_file(file: File) -> Vec<u32> {
    let mut reader = io::BufReader::new(file);
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Cannot read");
    s.lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>()
}

pub fn read_matrix(file: File) -> Vec<Vec<usize>> {
    let mut reader = BufReader::new(file);
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Cannot read");
    s.lines()
        .map(|line| {
            line.split(' ')
                .map(|char| char.parse().ok().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}
