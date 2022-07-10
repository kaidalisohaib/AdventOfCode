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
    let mut number: u32 = 1;
    loop {
        let key: String = format!("{}{}", input, number);
        let hash: String = format!("{:?}", md5::compute(&key));
        if hash.starts_with("00000") {
            return number;
        }
        number += 1;
    }
}

fn solve_part_two(input: &str) -> u32 {
    let mut number: u32 = 1;
    loop {
        let key: String = format!("{}{}", input, number);
        let hash: String = format!("{:?}", md5::compute(&key));
        if hash.starts_with("000000") {
            return number;
        }
        number += 1;
    }
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
