pub const TEST_FILE_PATH: &str = "test_input.txt";
pub const FILE_PATH: &str = "input.txt";

pub trait Puzzle {
    fn get_name(&self) -> &'static str;
    fn get_expected_test_result(&self) -> &str;
    fn get_result(&self, lines: Vec<String>) -> String;
}