use regex::{Captures, Regex};
use std::{collections::HashMap, fs, process};
#[derive(Debug)]
enum Instructions<'a> {
    hlf(&'a str), // divide by 2
    tpl(&'a str), // triple
    inc(&'a str), // increment by 1
    jmp(i32),
    jie(&'a str, i32), // if even
    jio(&'a str, i32), // if one
}

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
    let program_instructions: Vec<Instructions> = prepare_data(input);
    let mut registers: HashMap<&str, u32> = HashMap::from([("a", 0), ("b", 0)]);
    let mut current_line: i32 = 0;
    while current_line as usize != program_instructions.len() {
        let current_instruction: &Instructions = &program_instructions[current_line as usize];
        match *current_instruction {
            Instructions::hlf(register) => {
                let value = registers.get_mut(&register).unwrap();
                *value /= 2;
            }
            Instructions::tpl(register) => {
                let value = registers.get_mut(&register).unwrap();
                *value *= 3;
            }
            Instructions::inc(register) => {
                let value = registers.get_mut(&register).unwrap();
                *value += 1;
            }
            Instructions::jmp(offset) => {
                current_line += offset;
                continue;
            }
            Instructions::jie(register, offset) => {
                if *registers.get(register).unwrap() % 2 == 0 {
                    current_line += offset;
                    continue;
                }
            }
            Instructions::jio(register, offset) => {
                if *registers.get(register).unwrap() == 1 {
                    current_line += offset;
                    continue;
                }
            }
        }
        current_line += 1;
    }
    *registers.get("b").unwrap()
}

fn solve_part_two(input: &str)-> u32 {
    let program_instructions: Vec<Instructions> = prepare_data(input);
    let mut registers: HashMap<&str, u32> = HashMap::from([("a", 1), ("b", 0)]);
    let mut current_line: i32 = 0;
    while current_line as usize != program_instructions.len() {
        let current_instruction: &Instructions = &program_instructions[current_line as usize];
        match *current_instruction {
            Instructions::hlf(register) => {
                let value = registers.get_mut(&register).unwrap();
                *value /= 2;
            }
            Instructions::tpl(register) => {
                let value = registers.get_mut(&register).unwrap();
                *value *= 3;
            }
            Instructions::inc(register) => {
                let value = registers.get_mut(&register).unwrap();
                *value += 1;
            }
            Instructions::jmp(offset) => {
                current_line += offset;
                continue;
            }
            Instructions::jie(register, offset) => {
                if *registers.get(register).unwrap() % 2 == 0 {
                    current_line += offset;
                    continue;
                }
            }
            Instructions::jio(register, offset) => {
                if *registers.get(register).unwrap() == 1 {
                    current_line += offset;
                    continue;
                }
            }
        }
        current_line += 1;
    }
    *registers.get("b").unwrap()
}

fn prepare_data(input: &str) -> Vec<Instructions> {
    let mut instructions: Vec<Instructions> = Vec::new();
    let re: Regex = Regex::new(r"^(\w+) ([+,-]?\w+)(?:, ([-,+]\d+))?").unwrap();
    for line in input.lines() {
        let captures: Captures = re.captures(line).unwrap();
        let instruction_str: &str = captures.get(1).unwrap().as_str();
        let mut instruction: Option<Instructions> = None;
        match instruction_str {
            "hlf" => instruction = Some(Instructions::hlf(captures.get(2).unwrap().as_str())),
            "tpl" => instruction = Some(Instructions::tpl(captures.get(2).unwrap().as_str())),
            "inc" => instruction = Some(Instructions::inc(captures.get(2).unwrap().as_str())),
            "jmp" => {
                instruction = Some(Instructions::jmp(
                    captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ))
            }
            "jie" => {
                instruction = Some(Instructions::jie(
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                ))
            }
            "jio" => {
                instruction = Some(Instructions::jio(
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                ))
            }
            _ => panic!("Instruction not found in the list !!"),
        }
        instructions.push(instruction.unwrap());
    }

    return instructions;
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
