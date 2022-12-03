use std::{fs::File, io::{self, BufRead, BufReader}};

const FILE_PATH: &str = "input.txt";

pub fn input_lines() -> std::io::Lines<BufReader<File>> {
    let file = File::open(FILE_PATH)
        .expect("Could not open input.txt!");

    return io::BufReader::new(file).lines();
}