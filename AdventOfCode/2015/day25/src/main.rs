use std::{fs, process};
const ROW: u32 = 2978;
const COLUMN: u32 = 3083;
const START_POSITION: ((u32, u32), u128) = ((6, 6), 27995004);
fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_one_answer: u128 = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
}

fn solve_part_one(_input: &str) -> u128 {
    let mut current_column: u32 = START_POSITION.0 .0;
    let mut current_row: u32 = START_POSITION.0 .1;
    let mut last_value: u128 = START_POSITION.1;

    while current_column != COLUMN || current_row != ROW {
        if current_row == 1 {
            current_row = current_column + 1;
            current_column = 1;
        } else {
            current_row -= 1;
            current_column += 1;
        }
        last_value = calculate_next(last_value);
    }
    last_value
}

fn calculate_next(last_value: u128) -> u128 {
    (last_value * 252533) % 33554393
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
