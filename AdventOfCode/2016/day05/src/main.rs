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

fn solve_part_one(input: &str) -> String {
    let input: String = String::from(input.trim());
    let mut password: Vec<char> = Vec::new();
    for num in 0..u32::MAX {
        let encoded_text: String = format!(
            "{:x}",
            md5::compute(format!("{}{}", &input, &num.to_string()).as_bytes())
        );
        if encoded_text.starts_with("00000") {
            password.push(*encoded_text.chars().collect::<Vec<char>>().get(5).unwrap());
            if password.len() >= 8 {
                break;
            }
        }
    }
    password
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn solve_part_two(input: &str) -> String {
    let input: String = String::from(input.trim());
    let mut password: Vec<Option<char>> = vec![None; 8];
    for num in 0..u32::MAX {
        let encoded_text: String = format!(
            "{:x}",
            md5::compute(format!("{}{}", &input, &num.to_string()).as_bytes())
        );
        if encoded_text.starts_with("00000") {
            let encoded_text_chars: Vec<char> = encoded_text.chars().collect::<Vec<char>>();
            // password.push(*.get(5).unwrap());
            let index_option = encoded_text_chars
                .get(5)
                .unwrap()
                .to_string()
                .parse::<usize>();
            match index_option {
                Err(_) => continue,
                Ok(index) => {
                    if index >= password.len() {
                        continue;
                    }
                    match password[index_option.unwrap()] {
                        Some(_) => continue,
                        None => password[index] = Some(*encoded_text_chars.get(6).unwrap()),
                    }
                }
            }

            if password.iter().all(|x| x.is_some()) {
                break;
            }
        }
    }
    password
        .iter()
        .map(|&x| x.unwrap().to_string())
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
