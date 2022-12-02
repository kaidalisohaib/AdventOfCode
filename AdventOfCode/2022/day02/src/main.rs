use std::{fs, process};
// First column letter, second column letter, value
const ROCK_1: char = 'A';
const PAPER_1: char = 'B';
const SCISSORS_1: char = 'C';

const ROCK_2: char = 'X';
const PAPER_2: char = 'Y';
const SCISSORS_2: char = 'Z';

const ROCK_VALUE: u32 = 1;
const PAPER_VALUE: u32 = 2;
const SCISSORS_VALUE: u32 = 3;

const LOST_VALUE: u32 = 0;
const DRAW_VALUE: u32 = 3;
const WIN_VALUE: u32 = 6;

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
    let mut score: u32 = 0;
    for line in _input.lines() {
        let characters: Vec<char> = line.chars().collect::<Vec<char>>();
        let first_letter: char = *characters.first().unwrap();
        let second_letter: char = *characters.last().unwrap();
        match first_letter {
            ROCK_1 => match second_letter {
                ROCK_2 => score += ROCK_VALUE + DRAW_VALUE,
                PAPER_2 => score += PAPER_VALUE + WIN_VALUE,
                SCISSORS_2 => score += SCISSORS_VALUE + LOST_VALUE,
                _ => unreachable!(),
            },
            PAPER_1 => match second_letter {
                ROCK_2 => score += ROCK_VALUE + LOST_VALUE,
                PAPER_2 => score += PAPER_VALUE + DRAW_VALUE,
                SCISSORS_2 => score += SCISSORS_VALUE + WIN_VALUE,
                _ => unreachable!(),
            },
            SCISSORS_1 => match second_letter {
                ROCK_2 => score += ROCK_VALUE + WIN_VALUE,
                PAPER_2 => score += PAPER_VALUE + LOST_VALUE,
                SCISSORS_2 => score += SCISSORS_VALUE + DRAW_VALUE,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    score
}

fn solve_part_two(_input: &str) -> u32 {
    let mut score: u32 = 0;
    for line in _input.lines() {
        let characters: Vec<char> = line.chars().collect::<Vec<char>>();
        let first_letter: char = *characters.first().unwrap();
        let second_letter: char = *characters.last().unwrap();
        match first_letter {
            ROCK_1 => match second_letter {
                // NEED TO LOSE
                ROCK_2 => score += SCISSORS_VALUE + LOST_VALUE,
                // NEED TO DRAW
                PAPER_2 => score += ROCK_VALUE + DRAW_VALUE,
                // NEED TO WIN
                SCISSORS_2 => score += PAPER_VALUE + WIN_VALUE,
                _ => unreachable!(),
            },
            PAPER_1 => match second_letter {
                // NEED TO LOSE
                ROCK_2 => score += ROCK_VALUE + LOST_VALUE,
                // NEED TO DRAW
                PAPER_2 => score += PAPER_VALUE + DRAW_VALUE,
                // NEED TO WIN
                SCISSORS_2 => score += SCISSORS_VALUE + WIN_VALUE,
                _ => unreachable!(),
            },
            SCISSORS_1 => match second_letter {
                // NEED TO LOSE
                ROCK_2 => score += PAPER_VALUE + LOST_VALUE,
                // NEED TO DRAW
                PAPER_2 => score += SCISSORS_VALUE + DRAW_VALUE,
                // NEED TO WIN
                SCISSORS_2 => score += ROCK_VALUE + WIN_VALUE,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    score
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
