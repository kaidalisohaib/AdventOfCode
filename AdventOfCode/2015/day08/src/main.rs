use std::{fs, process};

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);

    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> u32 {
    let mut total_string_code: u32 = 0;
    let mut total_string_memory: u32 = 0;
    for line in input.lines() {
        total_string_code += line.len() as u32;
        let mut current_string_memory: u32 = 0;
        let mut index: usize = 1;
        let characters: Vec<char> = line.chars().collect();
        while index < characters.len() - 1 {
            current_string_memory += 1;
            let current_character: char = *characters.get(index).unwrap();
            if current_character == '\\' {
                let next_character: char = *characters.get(index + 1).unwrap();
                if next_character == 'x' {
                    index += 3;
                } else {
                    index += 1;
                }
            }

            index += 1;
        }
        total_string_memory += current_string_memory;
    }
    total_string_code - total_string_memory
}

fn solve_part_two(input: &str) -> u32 {
    let mut total_final_string: u32 = 0;
    let mut total_original_string: u32 = 0;
    for line in input.lines() {
        total_original_string += line.len() as u32;
        let mut line_string: String = String::from(line);
        let mut index: usize = 0;
        while index < line_string.len() {
            let current_character: &str = line_string.get(index..=index).unwrap();
            if current_character == "\"" || current_character == "\\" {
                line_string.insert(index, '\\');
                index += 1;
            }
            index += 1;
        }
        line_string.insert(0, '"');
        line_string.push('"');
        total_final_string += line_string.len() as u32;
    }
    total_final_string - total_original_string
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
