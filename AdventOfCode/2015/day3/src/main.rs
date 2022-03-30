use std::{collections::HashSet, fs, process};

const UP: char = '^';
const RIGHT: char = '>';
const DOWN: char = 'v';
const LEFT: char = '<';

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);

    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> usize {
    let mut position: (i32, i32) = (0, 0);
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert(position);
    for character in input.chars() {
        match character {
            UP => position.1 += 1,
            DOWN => position.1 -= 1,
            RIGHT => position.0 += 1,
            LEFT => position.0 -= 1,
            _ => {
                eprintln!("Unexpected character in input: {}", character);
                process::exit(1);
            }
        }
        positions.insert(position);
    }
    return positions.len();
}

fn solve_part_two(input: &str) -> usize {
    let mut position1: (i32, i32) = (0, 0);
    let mut position2: (i32, i32) = (0, 0);
    let mut position_transfert: &mut (i32, i32) = &mut position1;
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert(position1);
    positions.insert(position2);
    for (index, character) in input.chars().enumerate() {
        if index % 2 == 0 {
            position_transfert = &mut position1;
        } else {
            position_transfert = &mut position2;
        }
        match character {
            UP => position_transfert.1 += 1,
            DOWN => position_transfert.1 -= 1,
            RIGHT => position_transfert.0 += 1,
            LEFT => position_transfert.0 -= 1,
            _ => {
                eprintln!("Unexpected character in input: {}", character);
                process::exit(1);
            }
        }
        positions.insert(*position_transfert);
    }
    return positions.len();
}

fn read_input_file() -> String {
    match fs::read_to_string("input.txt") {
        Ok(content) => return content,
        Err(err) => {
            eprintln!("Error while opening the input file: {:?}", err);
            process::exit(1);
        }
    };
}
