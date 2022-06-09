use std::{fs, process};

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_one_answer= solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> usize {
    let mut last_value: String = String::from(input);
    for index in 0..40 {
        last_value = generate_look_and_say_seq(last_value.as_str());    
    }

    return last_value.len()
}

fn solve_part_two(input: &str) -> usize{
    let mut last_value: String = String::from(input);
    for index in 0..50 {
        last_value = generate_look_and_say_seq(last_value.as_str());    
    }

    return last_value.len()
}

fn generate_look_and_say_seq(number: &str) -> String{
    let mut final_string: String = String::new();
    let mut number_of_same_number: u32 = 0;
    let characters: Vec<char> = number.chars().collect();
    for index in 0..characters.len() {
        number_of_same_number += 1;
        if( (index < characters.len()-1 && characters[index] != characters[index+1]) || index == characters.len()-1){
            let number_of_same_number_string = number_of_same_number.to_string();
            final_string.push_str(&number_of_same_number_string.as_str());
            final_string.push(characters[index]);
            number_of_same_number = 0;
        }
    }
    return final_string;
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
