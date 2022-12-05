use lazy_static::lazy_static;
use regex::Regex;

pub const PUZZLE: Puzzle = Puzzle { name: "day5_2" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        return self.name;
    }

    fn get_expected_test_result(&self) -> &str {
        return "MCD";
    }

    fn get_result(&self, lines: Vec<String>) -> String {
        let mut stacks: Vec<Vec<char>> = vec![];
        populate_stacks(&lines, &mut stacks);
        'lines: for line in lines {
            let mut temp_stack: Vec<char> = vec![];
            let mv = parse_move(&line);
            if let Some(mv) = mv {
                for _ in 0..mv.n {
                    temp_stack.push(match stacks[mv.src as usize].pop() {
                        Some(val) => val,
                        None => continue 'lines
                    });
                }
                temp_stack.reverse();
                for c in temp_stack {
                    stacks[mv.dst as usize].push(c);
                }
            }
        }

        let mut out_str = String::new();
        for stack in stacks {
            out_str += stack.last().unwrap().to_string().as_str();
        }
        return out_str;
    }
}

fn populate_stacks(lines: &Vec<String>, stacks: &mut Vec<Vec<char>>) {
    let num_stacks = (lines[0].len() + 1) / 4;
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    for line in lines {
        if line.starts_with(" 1 ") {
            break;
        }
        for i in 0..num_stacks {
            let ch = line.chars().nth(4 * i + 1).unwrap();
            if ch == ' ' {
                continue;
            } else {
                stacks[i].push(ch);
            }
        }
    }

    for stack in stacks {
        stack.reverse();
    }
}

struct Move { n: u32, src: u32, dst: u32 }
fn parse_move(line: &String) -> Option<Move> {
    lazy_static! {
        static ref RE: Regex = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
    }
    if RE.is_match(line) {
        let captures = RE.captures(line).unwrap();
        let n = match match captures.get(1) {
            Some(val) => val,
            None => return None
        }.as_str().parse::<u32>() {
            Ok(val) => val,
            Err(_) => return None
        };
        let src = match match captures.get(2) {
            Some(val) => val,
            None => return None
        }.as_str().parse::<u32>() {
            Ok(val) => val,
            Err(_) => return None
        };
        let dst = match match captures.get(3) {
            Some(val) => val,
            None => return None
        }.as_str().parse::<u32>() {
            Ok(val) => val,
            Err(_) => return None
        };
        return Some(Move { n, src: src - 1, dst: dst - 1 });
    }

    return None;
}