use std::{collections::HashSet, fs, process};
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
    let mut number_of_tls: u32 = 0;
    for line in input.lines() {
        if support_tls(line) {
            number_of_tls += 1;
        }
    }
    number_of_tls
}

fn solve_part_two(input: &str) -> u32 {
    let mut number_of_ssl: u32 = 0;
    for line in input.lines() {
        if support_ssl(line) {
            number_of_ssl += 1;
        }
    }
    number_of_ssl
}

fn support_tls(ip: &str) -> bool {
    if ip.len() < 4 {
        return false;
    }
    let square_braquets: [char; 2] = ['[', ']'];
    let mut second_window_character: char = get_char(ip, 0).unwrap();
    let mut third_window_character: char = get_char(ip, 0).unwrap();
    let mut fourth_window_character: char = get_char(ip, 0).unwrap();
    let ip: String = format!("{}{}", fourth_window_character.to_string().repeat(3), ip);
    let mut skip_check_until: usize = 2;
    let mut between_square_braquets: bool = false;
    let mut contains_tls_outside_square_braquets: bool = false;
    for index in 0..(ip.len() - 3) as usize {
        let first_window_character: char = second_window_character;
        second_window_character = third_window_character;
        third_window_character = fourth_window_character;
        fourth_window_character = get_char(&ip, index + 3).unwrap();
        if square_braquets.contains(&fourth_window_character) {
            skip_check_until = index + 3;
            between_square_braquets = !between_square_braquets;
        }
        if index <= skip_check_until {
            continue;
        }
        if first_window_character == fourth_window_character
            && second_window_character == third_window_character
            && first_window_character != second_window_character
        {
            if !between_square_braquets {
                if !contains_tls_outside_square_braquets {
                    contains_tls_outside_square_braquets = true;
                }
            } else {
                return false;
            }
        }
    }
    contains_tls_outside_square_braquets
}

fn support_ssl(ip: &str) -> bool {
    if ip.len() < 3 {
        return false;
    }
    let square_braquets: [char; 2] = ['[', ']'];
    let mut second_window_character: char = get_char(ip, 0).unwrap();
    let mut third_window_character: char = get_char(ip, 0).unwrap();
    let ip: String = format!("{}{}", third_window_character.to_string().repeat(2), ip);
    let mut skip_check_until: usize = 1;
    let mut between_square_braquets: bool = false;
    let mut all_sequences: HashSet<((char, char), bool)> = HashSet::new(); // (A,B)A and betwwn_square_braquet
    for index in 0..(ip.len() - 2) as usize {
        let first_window_character: char = second_window_character;
        second_window_character = third_window_character;
        third_window_character = get_char(&ip, index + 2).unwrap();
        if square_braquets.contains(&third_window_character) {
            skip_check_until = index + 2;
            between_square_braquets = !between_square_braquets;
        }
        if index <= skip_check_until {
            continue;
        }
        if first_window_character == third_window_character {
            if all_sequences.contains(&(
                (second_window_character, first_window_character),
                !between_square_braquets,
            )) {
                return true;
            }
            all_sequences.insert((
                (first_window_character, second_window_character),
                between_square_braquets,
            ));
        }
    }
    false
}

fn get_char(text: &str, wanted_index: usize) -> Option<char> {
    if wanted_index >= text.len() {
        return None;
    }
    if let Some((_, character)) = text
        .char_indices()
        .find(|(index, _)| *index == wanted_index)
    {
        return Some(character);
    }
    None
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
