use std::{fs, process};

const NOT_ALLOWED_LETTER: [u16; 3] = ['i' as u16 - 97, 'o' as u16 - 97, 'l' as u16 - 97];
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
    let mut characters: Vec<u16> = input.encode_utf16().map(|x| x - 97).collect();
    if is_password_secure(&characters) {
        increment_password_by_1(&mut characters);
    }
    skip_non_allowed_letter(&mut characters);
    while !is_password_secure(&characters) {
        for index in 0..characters.len() {
            if NOT_ALLOWED_LETTER.contains(&characters[index]) {
                skip_non_allowed_letter(&mut characters);
            }
        }
        increment_password_by_1(&mut characters);
    }
    return characters
        .into_iter()
        .map(|x| String::from(((x + 97) as u8) as char))
        .collect::<Vec<String>>()
        .join("");
}

fn solve_part_two(input: &str) -> String {
    let mut characters: Vec<u16> = input.encode_utf16().map(|x| x - 97).collect();
    for two_time in 0..2 {
        if is_password_secure(&characters) {
            increment_password_by_1(&mut characters);
        }
        skip_non_allowed_letter(&mut characters);
        while !is_password_secure(&characters) {
            for index in 0..characters.len() {
                if NOT_ALLOWED_LETTER.contains(&characters[index]) {
                    skip_non_allowed_letter(&mut characters);
                }
            }
            increment_password_by_1(&mut characters);
        }
    }
    return characters
        .into_iter()
        .map(|x| String::from(((x + 97) as u8) as char))
        .collect::<Vec<String>>()
        .join("");
}

fn increment_password_by_1(password: &mut Vec<u16>) {
    if password.is_empty() {
        return;
    }
    for index in (0..password.len()).rev() {
        password[index] += 1;
        while NOT_ALLOWED_LETTER.contains(&password[index]) {
            password[index] += 1;
        }
        if password[index] >= 26 {
            password[index] = 0;
            continue;
        }
        break;
    }
}

fn skip_non_allowed_letter(password: &mut Vec<u16>) {
    if password.is_empty() {
        return;
    }
    let mut biggest_index_nal: Option<usize> = None; //not allowed letter "nal"
    for index in (0..password.len()).rev() {
        if NOT_ALLOWED_LETTER.contains(&password[index]) {
            biggest_index_nal = Some(index);
        }
    }
    if let Some(biggest_index_value) = biggest_index_nal {
        for index in biggest_index_value + 1..password.len() {
            password[index] = 0;
        }
        password[biggest_index_value] += 1;
    }
}

fn is_password_secure(password: &Vec<u16>) -> bool {
    for character in password {
        if NOT_ALLOWED_LETTER.contains(character) {
            return false;
        }
    }

    let mut index_second: usize = 0;
    let mut first_repetetive_letter: Option<u16> = None;
    let mut number_of_repetetive_letter: u16 = 0;
    while index_second < password.len() - 1 {
        if password[index_second] == password[index_second + 1]
            && (first_repetetive_letter.is_none()
                || (!first_repetetive_letter.is_none()
                    && first_repetetive_letter.unwrap() != password[index_second]))
        {
            first_repetetive_letter = Some(password[index_second]);
            index_second += 2;
            number_of_repetetive_letter += 1;
            if number_of_repetetive_letter == 2 {
                break;
            }
        } else {
            index_second += 1;
        }
    }
    if number_of_repetetive_letter < 2 {
        return false;
    }

    for index in 0..password.len() - 2 {
        let current_character: u16 = password[index];
        if password[index + 1] == current_character + 1
            && password[index + 2] == current_character + 2
        {
            return true;
        }
    }

    return false;
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
