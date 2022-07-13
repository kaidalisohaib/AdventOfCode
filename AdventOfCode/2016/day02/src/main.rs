use std::{fs, process};
const KEYPAD_1: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
const KEYPAD_2: [[Option<char>; 5]; 5] = [
    [None, None, Some('1'), None, None],
    [None, Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None, Some('A'), Some('B'), Some('C'), None],
    [None, None, Some('D'), None, None],
];
fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> String {
    // Starting at 5
    let mut current_pos: (i8, i8) = (1, 1);
    let mut password: Vec<u8> = Vec::new();

    for line in input.lines() {
        for character in line.chars() {
            match character {
                'U' => {
                    current_pos.0 = i8::clamp(
                        current_pos.0 - 1,
                        0,
                        (KEYPAD_1.len() - 1).try_into().unwrap(),
                    )
                }
                'D' => {
                    current_pos.0 = i8::clamp(
                        current_pos.0 + 1,
                        0,
                        (KEYPAD_1.len() - 1).try_into().unwrap(),
                    )
                }
                'R' => {
                    current_pos.1 = i8::clamp(
                        current_pos.1 + 1,
                        0,
                        (KEYPAD_1[current_pos.0 as usize].len() - 1)
                            .try_into()
                            .unwrap(),
                    )
                }
                'L' => {
                    current_pos.1 = i8::clamp(
                        current_pos.1 - 1,
                        0,
                        (KEYPAD_1[current_pos.0 as usize].len() - 1)
                            .try_into()
                            .unwrap(),
                    )
                }
                _ => panic!("Impossible character given the situation!"),
            }
        }
        password.push(KEYPAD_1[current_pos.0 as usize][current_pos.1 as usize]);
    }
    password
        .iter()
        .map(|x| (*x).to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn solve_part_two(input: &str) -> String {
    // Starting at 5
    let mut current_pos: (i8, i8) = (2, 0);
    let mut password: Vec<char> = Vec::new();

    for line in input.lines() {
        for character in line.chars() {
            match character {
                'U' => {
                    if current_pos.0 > 0
                        && KEYPAD_2[(current_pos.0 - 1) as usize][current_pos.1 as usize].is_some()
                    {
                        current_pos.0 -= 1;
                    }
                }
                'D' => {
                    if current_pos.0 + 1 < KEYPAD_2.len().try_into().unwrap()
                        && KEYPAD_2[(current_pos.0 + 1) as usize][current_pos.1 as usize].is_some()
                    {
                        current_pos.0 += 1;
                    }
                }
                'R' => {
                    if current_pos.1 + 1
                        < KEYPAD_2[current_pos.0 as usize].len().try_into().unwrap()
                        && KEYPAD_2[current_pos.0 as usize][(current_pos.1 + 1) as usize].is_some()
                    {
                        current_pos.1 += 1;
                    }
                }
                'L' => {
                    if current_pos.1 > 0
                        && KEYPAD_2[current_pos.0 as usize][(current_pos.1 - 1) as usize].is_some()
                    {
                        current_pos.1 -= 1;
                    }
                }
                _ => panic!("Impossible character given the situation!"),
            }
        }
        password.push(KEYPAD_2[current_pos.0 as usize][current_pos.1 as usize].unwrap());
    }
    password
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
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
