use std::{fs, process};

const UP_FLOOR_CHAR: char = '(';
const DOWN_FLOOR_CHAR: char = ')';

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);

    let part_one_answer: i32 = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer: usize = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for character in input.chars() {
        match character {
            UP_FLOOR_CHAR => floor += 1,
            DOWN_FLOOR_CHAR => floor -= 1,
            _ => {
                eprintln!("Unexpected character in input: {:?}", character);
                process::exit(1);
            }
        }
    }
    floor
}

fn solve_part_two(input: &str) -> usize {
    let mut floor: i32 = 0;
    for (index, character) in input.char_indices() {
        match character {
            UP_FLOOR_CHAR => floor += 1,
            DOWN_FLOOR_CHAR => floor -= 1,
            _ => {
                eprintln!("Unexpected character in input: {:?}", character);
                process::exit(1);
            }
        }
        if floor == -1 {
            return index + 1;
        }
    }
    eprintln!("Didn't found an answer for part two");
    process::exit(1);
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
