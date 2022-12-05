use std::ops::Range;

use regex::Regex;
use lazy_static::lazy_static;

pub const PUZZLE: Puzzle = Puzzle { name: "day4_1" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_expected_test_result(&self) -> &str {
        return "2";
    }

    fn get_result(&self, lines: Vec<String>) -> String {
        let mut result = 0;
        for line in lines {
            if let Some(pair) = parse_line(&line) {
                if range_contains(&pair.0, &pair.1) || range_contains(&pair.1, &pair.0) {
                    result += 1;
                }
            }
        }

        return result.to_string();
    }
}

fn parse_line(line: &String) -> Option<(Range<u32>, Range<u32>)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
    }
    if !RE.is_match(line) {
        return None;
    }

    let captures = RE.captures(line).unwrap();
    if captures.len() != 5 {
        return None;
    }

    let left_1 = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let left_2 = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let right_1 = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let right_2 = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();

    return Some((left_1..left_2, right_1..right_2));
}

fn range_contains(a: &Range<u32>, b: &Range<u32>) -> bool {
    b.start >= a.start && b.end <= a.end
}