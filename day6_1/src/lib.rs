pub const PUZZLE: Puzzle = Puzzle { name: "day6_1" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        return self.name;
    }

    fn get_expected_test_result(&self) -> &str {
        "7"
    }

    fn get_result(&self, lines: Vec<String>) -> String {
        let line = lines.get(0).expect("Could not find line 0 of input!");
        let mut start: Option<usize> = None;
        for i in 0..(line.len() - 4) {
            if is_slice_marker(&line[i..(i + 4)]) {
                start = Some(i);
                break;
            }
        }

        return match start {
            Some(val) => (val + 4).to_string(),
            None => "No marker found!".to_string()
        }
    }
}

fn is_slice_marker(slice: &str) -> bool {
    for i in 0..slice.len() {
        for j in 0..i {
            if i != j && slice.chars().nth(i).unwrap() == slice.chars().nth(j).unwrap() {
                return false;
            }
        }
    }

    return true;
}