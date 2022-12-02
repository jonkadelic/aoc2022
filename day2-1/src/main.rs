use std::{fs::File, io::{self, BufRead}};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    ROCK,
    PAPER,
    SCISSORS
}

impl Shape {
    fn score(&self) -> u8 {
        match self {
            Shape::ROCK => 1,
            Shape::PAPER => 2,
            Shape::SCISSORS => 3
        }
    }

    fn left_char(&self) -> char {
        match self {
            Shape::ROCK => 'A',
            Shape::PAPER => 'B',
            Shape::SCISSORS => 'C'
        }
    }

    fn right_char(&self) -> char {
        match self {
            Shape::ROCK => 'X',
            Shape::PAPER => 'Y',
            Shape::SCISSORS => 'Z'
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Shape::ROCK => "ROCK",
            Shape::PAPER => "PAPER",
            Shape::SCISSORS => "SCISSORS"
        }
    }

    fn beats(&self, other: Shape) -> bool {
        match self {
            Shape::ROCK => other == Shape::SCISSORS,
            Shape::PAPER => other == Shape::ROCK,
            Shape::SCISSORS => other == Shape::PAPER
        }
    }

    fn from_char(val: char) -> Option<Shape> {
        let types = [Shape::ROCK, Shape::PAPER, Shape::SCISSORS];
        for t in types {
            if t.left_char() == val || t.right_char() == val {
                return Some(t);
            }
        }

        return None;
    } 
}

struct Contest {
    our_play: Shape,
    other_play: Shape
}

impl Contest {
    fn score(&self) -> u32 {
        if self.our_play == self.other_play {
            // Draw
            return 3 + self.our_play.score() as u32;
        } else if self.our_play.beats(self.other_play) {
            // We win
            return 6 + self.our_play.score() as u32;
        } else {
            // Other wins
            return 0 + self.our_play.score() as u32;
        }
    }
}

const FILE_PATH: &str = "input.txt";

fn main() {
    let mut contests: Vec<Contest> = vec![];

    let file = File::open(FILE_PATH)
        .expect("Could not open input.txt!");

    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(val) = line {
            let left = val.chars().nth(0).unwrap();
            let right = val.chars().nth(2).unwrap();
            let left_shape = Shape::from_char(left).unwrap();
            let right_shape = Shape::from_char(right).unwrap();
            contests.push(Contest { our_play: right_shape, other_play: left_shape })
        }
    }

    let mut total = 0;
    for contest in contests {
        let winner = if contest.our_play.beats(contest.other_play) {
            &contest.our_play
        } else {
            &contest.other_play
        };
        let score = contest.score();
        println!("{} vs {}, {} wins (score {})", 
            contest.our_play.to_string(), 
            contest.other_play.to_string(),
            winner.to_string(),
            score
        );
        total = total + score
    }
    println!();
    println!("Total score: {}", total);
}
