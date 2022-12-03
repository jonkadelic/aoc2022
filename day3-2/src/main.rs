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

fn find_common_item(packs: &[String; 3]) -> Option<char> {
    if packs.len() != 3 {
        return None;
    }
    for c1 in packs[0].chars() {
        for c2 in packs[1].chars() {
            for c3 in packs[2].chars() {
                if c1 == c2 && c2 == c3 && c1 == c3 {
                    return Some(c1);
                }
            }
        }
    }

    return None;
}

fn main() {
    let lines = crate::input::input_lines();
    let mut sum: u32 = 0;
    let mut group: Vec<String> = vec![];
    for line in lines {
        if let Ok(val) = line {
            if group.len() < 3 {
                group.push(val.clone());
                if group.len() < 3 {
                    continue
                }
            }
            let common = find_common_item(group[0..3].try_into().expect("Could not convert to [String; 3]!")).unwrap();
            sum += get_priority(common).unwrap();
            group.clear()
        }
    }

    println!("{}", sum);
}
