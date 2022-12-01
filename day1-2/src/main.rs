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

    // Sort elves
    elf_calories.sort_by(|a, b| u32::cmp(&(b.iter().sum()), &(a.iter().sum())));

    for elf in elf_calories.iter().enumerate() {
        println!("Elf {}:", elf.0 + 1);
        let mut total = 0;
        for calorie in elf.1.iter().enumerate() {
            println!("{}: {} calories", calorie.0 + 1, calorie.1);
            total = total + calorie.1;
        }
        println!("Total: {} calories", total);
        println!();
    }

    let sum: u32 = elf_calories[0].iter().sum::<u32>() + elf_calories[1].iter().sum::<u32>() + elf_calories[2].iter().sum::<u32>();
    println!("The top 3 elves are carrying a total of {} calories.", sum);

}
