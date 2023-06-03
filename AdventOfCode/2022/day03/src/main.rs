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

fn solve_part_one(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        // For each letter store if it was first found in the first or second compartment
        // true for first, false for second
        let mut letters_check: [Option<bool>; 26 * 2] = [None; 26 * 2];
        let midpoint_index: usize = (line.len() / 2) - 1;
        let mut first_compartment: bool = true;
        'inner: for (index, letter) in line.chars().enumerate() {
            let priority: u32 = letter_to_priority(letter);
            match letters_check[priority as usize - 1] {
                Some(compartment) => {
                    if compartment != first_compartment {
                        sum += priority;
                        break 'inner;
                    }
                }
                None => letters_check[priority as usize - 1] = Some(first_compartment),
            }

            if index == midpoint_index {
                first_compartment = false;
            }
        }
    }
    sum
}

fn solve_part_two(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    'outer: for group in lines.chunks(3) {
        let mut letters_check: [[bool; 26 * 2]; 3] = [[false; 26 * 2]; 3];
        for (bag_index, bag) in group.iter().enumerate() {
            for letter in bag.chars() {
                let letter_priority_index: u32 = letter_to_priority(letter) - 1;
                if bag_index == 2 {
                    if letters_check[0][letter_priority_index as usize]
                        && letters_check[1][letter_priority_index as usize]
                    {
                        sum += letter_priority_index + 1;
                        continue 'outer;
                    }
                } else {
                    letters_check[bag_index][letter_priority_index as usize] = true;
                }
            }
        }
    }
    sum
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

fn letter_to_priority(letter: char) -> u32 {
    let priority: u32 = letter as u32;
    if priority > 96 {
        // For lower case letters
        return priority - 96;
    }
    // For upper case letters
    priority - 38
}
