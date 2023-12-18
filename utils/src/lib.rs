use std::fs::File;
use std::io;
use std::io::Read;

pub const FILE_NOT_FOUND_MSG: &str = "Can't find a file";

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_numbers_from_file(file: File) -> Vec<u32> {
    let mut reader = io::BufReader::new(file);
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Cannot read");
    s.lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
