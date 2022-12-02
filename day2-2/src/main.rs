use std::{fs::File, io::{self, BufRead}, string};

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

    fn char(&self) -> char {
        match self {
            Shape::ROCK => 'A',
            Shape::PAPER => 'B',
            Shape::SCISSORS => 'C'
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Shape::ROCK => "ROCK",
            Shape::PAPER => "PAPER",
            Shape::SCISSORS => "SCISSORS"
        }
    }

    fn beats(&self) -> Shape {
        match self {
            Shape::ROCK => Shape::SCISSORS,
            Shape::PAPER => Shape::ROCK,
            Shape::SCISSORS => Shape::PAPER
        }
    }

    fn is_beaten_by(&self) -> Shape {
        match self {
            Shape::ROCK => Shape::PAPER,
            Shape::PAPER => Shape::SCISSORS,
            Shape::SCISSORS => Shape::ROCK
        }
    }

    fn from_char(val: char) -> Option<Shape> {
        let types = [Shape::ROCK, Shape::PAPER, Shape::SCISSORS];
        for t in types {
            if t.char() == val {
                return Some(t);
            }
        }

        return None;
    } 
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Move {
    LOSE,
    DRAW,
    WIN
}

impl Move {
    fn get_shape(&self, other: Shape) -> Shape {
        match self {
            Move::LOSE => other.beats(),
            Move::DRAW => other,
            Move::WIN => other.is_beaten_by()
        }
    }

    fn score(&self) -> u8 {
        match self {
            Move::LOSE => 0,
            Move::DRAW => 3,
            Move::WIN => 6
        }
    }

    fn char(&self) -> char {
        match self {
            Move::LOSE => 'X',
            Move::DRAW => 'Y',
            Move::WIN => 'Z'
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Move::LOSE => "LOSE",
            Move::DRAW => "DRAW",
            Move::WIN => "WIN"
        }
    }

    fn from_char(val: char) -> Option<Move> {
        let moves = [Move::LOSE, Move::DRAW, Move::WIN];
        for m in moves {
            if m.char() == val {
                return Some(m);
            }
        }

        return None;
    }
}

struct Contest {
    our_move: Move,
    other_play: Shape
}

impl Contest {
    fn score(&self) -> u32 {
        let our_play = self.our_move.get_shape(self.other_play);
        return (our_play.score() + self.our_move.score()) as u32;
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
            let other_play = Shape::from_char(left).unwrap();
            let our_move = Move::from_char(right).unwrap();
            contests.push(Contest { our_move, other_play })
        }
    }

    let mut total = 0;
    for contest in contests {
        let our_play = contest.our_move.get_shape(contest.other_play);
        let score = contest.score();
        println!("Other plays {} and we need to {}, so we play {} (score {})",
            contest.other_play.to_string(),
            contest.our_move.to_string(),
            our_play.to_string(),
            score
        );
        total += score;
    }
    println!();
    println!("Total score: {}", total);
}
