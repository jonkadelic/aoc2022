use std::{fs::File, io::{self, BufRead, BufReader}};

pub const TEST_FILE_PATH: &str = "test_input.txt";
pub const FILE_PATH: &str = "input.txt";

pub trait Puzzle {
    fn get_name(&self) -> &'static str;
    fn get_expected_test_result(&self) -> u32;
    fn get_result(&self, path: &str) -> u32;
}

pub fn input_lines(path: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(path)
        .expect(format!("Could not open {}!", path).as_str());

    return io::BufReader::new(file).lines();
}