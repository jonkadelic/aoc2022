mod input;

fn get_priority(val: char) -> Option<u32> {
    if val as usize > 127 {
        return None;
    }
    if ('a'..='z').contains(&val) {
        return Some((val as u8 - 97 + 1) as u32);
    } else if ('A'..='Z').contains(&val) {
        return Some((val as u8 - 65 + 1 + 26) as u32);
    } else {
        return None;
    }
}

fn find_common_priority(left: &str, right: &str) -> Option<u32> {
    for lc in left.chars() {
        for rc in right.chars() {
            if lc == rc {
                return Some(get_priority(lc).unwrap());
            }
        }
    }

    return None;
}

fn main() {
    let lines = crate::input::input_lines();
    let mut sum: u32 = 0;
    for line in lines {
        if let Ok(val) = line {
            let (left, right) = val.split_at(val.len() / 2);
            sum = sum + find_common_priority(left, right).unwrap();
        }
    }

    println!("{}", sum);
}
