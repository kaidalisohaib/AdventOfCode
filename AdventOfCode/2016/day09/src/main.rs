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

fn solve_part_one(input: &str) -> usize {
    let mut current_index: usize = 0;
    let mut length: usize = 0;
    while current_index < input.len() {
        let current_character: char = char::from(*input.as_bytes().get(current_index).unwrap());
        if current_character != '(' {
            current_index += 1;
            length += 1;
            continue;
        }
        // find the two arguments
        let mut index_closing_parenthese: usize = current_index + 1;
        while char::from(*input.as_bytes().get(index_closing_parenthese).unwrap()) != ')' {
            index_closing_parenthese += 1;
        }
        let arguments: Vec<usize> = input
            .get(current_index + 1..index_closing_parenthese)
            .unwrap()
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        length += arguments.get(0).unwrap() * arguments.get(1).unwrap();
        current_index = index_closing_parenthese + 1 + arguments.get(0).unwrap();
    }
    length
}

fn solve_part_two(input: &str) -> usize {
    let mut current_index: usize = 0;
    let mut length: usize = 0;
    while current_index < input.len() {
        let current_character: char = char::from(*input.as_bytes().get(current_index).unwrap());
        if current_character != '(' {
            current_index += 1;
            length += 1;
            continue;
        }
        // find the two arguments
        let mut index_closing_parenthese: usize = current_index + 1;
        while char::from(*input.as_bytes().get(index_closing_parenthese).unwrap()) != ')' {
            index_closing_parenthese += 1;
        }
        let arguments: Vec<usize> = input
            .get(current_index + 1..index_closing_parenthese)
            .unwrap()
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let unpacked_section_length: usize = solve_part_two(
            input
                .get(
                    index_closing_parenthese + 1
                        ..=index_closing_parenthese + arguments.get(0).unwrap(),
                )
                .unwrap(),
        );

        length += unpacked_section_length * arguments.get(1).unwrap();
        current_index = index_closing_parenthese + 1 + arguments.get(0).unwrap();
    }
    length
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
