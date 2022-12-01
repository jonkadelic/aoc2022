use std::fs::File;
use std::io::{self, BufRead};

const FILE_PATH: &str = "input.txt";

fn main() {
    let mut elf_calories: Vec<Vec<u32>> = vec![];

    let file = File::open(FILE_PATH)
        .expect("Could not open input.txt!");
    
    let lines = io::BufReader::new(file).lines();

    let mut current_vec: Vec<u32> = vec![];
    for line in lines {
        if let Ok(val) = line {
            if val == "" {
                elf_calories.push(current_vec);
                current_vec = vec![];
            } else if let Ok(int) = val.parse::<u32>() {
                current_vec.push(int)
            }
        }
    }

    let mut highest_calories: (usize, u32) = (0, 0);
    for elf in elf_calories.iter().enumerate() {
        println!("Elf {}:", elf.0 + 1);
        let mut total = 0;
        for calorie in elf.1.iter().enumerate() {
            println!("{}: {} calories", calorie.0 + 1, calorie.1);
            total = total + calorie.1;
        }
        println!("Total: {} calories", total);
        println!();

        if total > highest_calories.1 {
            highest_calories = (elf.0, total);
        }
    }

    println!("Elf {} is carrying the most, at {} calories.", highest_calories.0 + 1, highest_calories.1);
}
