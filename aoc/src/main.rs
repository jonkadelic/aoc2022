use std::{io::{Lines, BufReader, BufRead}, fs::File};

const PUZZLES: [&dyn aoc_common::Puzzle; 2] = [
    &day4_1::PUZZLE as &dyn aoc_common::Puzzle,
    &day4_2::PUZZLE as &dyn aoc_common::Puzzle
];

fn get_puzzle_path(puzzle: &dyn aoc_common::Puzzle, test: bool) -> String {
    format!("input/{}/{}.txt", puzzle.get_name().split('_').nth(0).unwrap(), match test {
        false => "input",
        true => "test_input"
    })
}

pub fn get_input_lines(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path)
        .expect(format!("Could not open {}!", path).as_str());

    return BufReader::new(file).lines();
}

fn main() {
    let mut args = std::env::args();
    let puzzle = args.nth(1).expect("Please specify a puzzle!");
    let found_puzzle = PUZZLES.iter().find(|i| -> bool {
        i.get_name() == puzzle
    }).expect("Please specify a puzzle!");
    let lines = get_input_lines(get_puzzle_path(*found_puzzle, false).as_str());
    let result = found_puzzle.get_result(lines);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use yare::parameterized;

    use crate::{PUZZLES, get_puzzle_path, get_input_lines};

    #[parameterized(
        day4_1 = { PUZZLES[0] },
        day4_2 = { PUZZLES[1] }
    )]
    fn test_puzzle(puzzle: &dyn aoc_common::Puzzle) {
        let expected = puzzle.get_expected_test_result();
        let lines = get_input_lines(get_puzzle_path(puzzle, true).as_str());
        let result = puzzle.get_result(lines);
            assert_eq!(expected, result);
    }
}