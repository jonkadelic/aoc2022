use std::{ops::Range, io::{Lines, BufReader}, fs::File};

use regex::Regex;
use lazy_static::lazy_static;

pub const PUZZLE: Puzzle = Puzzle { name: "day4_2" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_expected_test_result(&self) -> u32 {
        return 4;
    }

    fn get_result(&self, lines: Lines<BufReader<File>>) -> u32 {
        let mut result = 0;
        for line in lines {
            if let Ok(val) = line {
                if let Some(pair) = parse_line(&val) {
                    if range_overlaps(&pair.0, &pair.1) || range_overlaps(&pair.1, &pair.0) {
                        result += 1;
                    }
                }
            }
        }

        return result;
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

fn range_overlaps(a: &Range<u32>, b: &Range<u32>) -> bool {
    (b.start >= a.start && b.end <= a.end) ||
        (b.end >= a.start && b.end <= a.end)
}