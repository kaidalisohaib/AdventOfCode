use std::{fs, process};

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

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
    let mut small_screen: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
    for instruction in input.lines() {
        let word_in_instruction: Vec<&str> = instruction.split(' ').collect::<Vec<&str>>();
        match word_in_instruction.len() {
            2 => {
                let dimension: Vec<usize> = word_in_instruction
                    .get(1)
                    .unwrap()
                    .split('x')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                rect(
                    &mut small_screen,
                    *dimension.get(0).unwrap(),
                    *dimension.get(1).unwrap(),
                )
            }
            5 => {
                let arguments: (&str, &str) = (
                    word_in_instruction
                        .get(2)
                        .unwrap()
                        .split('=')
                        .collect::<Vec<&str>>()
                        .get(1)
                        .unwrap(),
                    word_in_instruction.get(4).unwrap(),
                );
                match *word_in_instruction.get(1).unwrap() {
                    "row" => rotate_row(
                        &mut small_screen,
                        arguments.0.parse::<usize>().unwrap(),
                        arguments.1.parse::<u32>().unwrap(),
                    ),
                    "column" => rotate_column(
                        &mut small_screen,
                        arguments.0.parse::<usize>().unwrap(),
                        arguments.1.parse::<u32>().unwrap(),
                    ),
                    _ => panic!("Number of argument impossible given the situation!"),
                }
            }
            _ => panic!("Number of argument impossible given the situation!"),
        }
    }
    print_screen(&small_screen);
    small_screen
        .iter()
        .map(|&x| x.iter().filter(|&&y| y).count())
        .sum::<usize>()
}

fn solve_part_two(_input: &str) -> u32 {
    1
}

fn rect(small_screen: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    for row in small_screen.iter_mut().take(y) {
        for cell in row.iter_mut().take(x) {
            *cell = true;
        }
    }
}

fn rotate_row(small_screen: &mut [[bool; WIDTH]; HEIGHT], y: usize, number: u32) {
    let mut new_row: [bool; WIDTH] = [false; WIDTH];
    for (index_x, &light_state) in small_screen.get(y).unwrap().iter().enumerate() {
        let new_index: usize = (index_x + number as usize) % WIDTH;
        new_row[new_index] = light_state;
    }
    small_screen[y] = new_row;
}

fn rotate_column(small_screen: &mut [[bool; WIDTH]; HEIGHT], x: usize, number: u32) {
    let mut new_column: [bool; HEIGHT] = [false; HEIGHT];
    for (index_y, &light_state) in small_screen
        .iter()
        .map(|row| row.get(x).unwrap())
        .enumerate()
    {
        let new_index: usize = (index_y + number as usize) % HEIGHT;
        new_column[new_index] = light_state;
    }
    small_screen
        .iter_mut()
        .enumerate()
        .for_each(|(index, row)| row[x] = new_column[index]);
}

fn print_screen(small_screen: &[[bool; WIDTH]; HEIGHT]) {
    for row in small_screen {
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!()
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
