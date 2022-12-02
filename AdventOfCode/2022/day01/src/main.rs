use std::{fs, process};

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(_input: &str) -> u32 {
    let mut most_calories: u32 = 0;
    let mut current_elf_calories: u32 = 0;
    for line in _input.lines() {
        match line.parse::<u32>() {
            Ok(calories) => current_elf_calories += calories,
            Err(_) => {
                if current_elf_calories > most_calories {
                    most_calories = current_elf_calories;
                }
                current_elf_calories = 0;
            }
        }
    }
    if current_elf_calories > most_calories {
        most_calories = current_elf_calories;
    }
    most_calories
}

fn solve_part_two(_input: &str) -> u32 {
    let mut most_calories_1: u32 = 0;
    let mut most_calories_2: u32 = 0;
    let mut most_calories_3: u32 = 0;
    let mut current_elf_calories: u32 = 0;
    for line in _input.lines() {
        match line.parse::<u32>() {
            Ok(calories) => current_elf_calories += calories,
            Err(_) => {
                if current_elf_calories > most_calories_1 {
                    most_calories_3 = most_calories_2;
                    most_calories_2 = most_calories_1;
                    most_calories_1 = current_elf_calories;
                } else if current_elf_calories > most_calories_2 {
                    most_calories_3 = most_calories_2;
                    most_calories_2 = current_elf_calories;
                } else if current_elf_calories > most_calories_3 {
                    most_calories_3 = current_elf_calories;
                }
                current_elf_calories = 0;
            }
        }
    }
    if current_elf_calories > most_calories_1 {
        most_calories_3 = most_calories_2;
        most_calories_2 = most_calories_1;
        most_calories_1 = current_elf_calories;
    } else if current_elf_calories > most_calories_2 {
        most_calories_3 = most_calories_2;
        most_calories_2 = current_elf_calories;
    } else if current_elf_calories > most_calories_3 {
        most_calories_3 = current_elf_calories;
    }
    most_calories_1 + most_calories_2 + most_calories_3
}

fn read_input_file() -> String {
    match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error while opening the input file: {:?}", err);
            process::exit(1);
        }
    }
}
