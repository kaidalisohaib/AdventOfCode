use json::JsonValue;
use regex::{Regex};
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

fn solve_part_one(input: &str) -> i32 {
    let re: Regex = Regex::new(r"-?\d+").unwrap();
    return re
        .find_iter(input)
        .map(|match_obj| {
            input
                .get(match_obj.start()..match_obj.end())
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .sum();
}

fn solve_part_two(input: &str) -> i32{
    let parsed: json::JsonValue = json::parse(input).unwrap();
    let mut sum: i32 = 0;
    recusively_add(&mut sum, &parsed);
    sum
}

fn recusively_add(pointer: &mut i32, json_val: &JsonValue){
    if json_val.is_object(){
        for (key, value) in json_val.entries(){
            if value.is_string() && value == "red"{
                return;
            }
        }
        for (key, value) in json_val.entries(){
            if value.is_number(){
                *pointer += value.as_i32().unwrap();
            }else if value.is_array() || value.is_object(){
                recusively_add(pointer, value);
            }
        }
    }else if json_val.is_array(){
        for value in json_val.members() {
            if value.is_number(){
                *pointer += value.as_i32().unwrap();
            }else if value.is_array() || value.is_object(){
                recusively_add(pointer, value);
            }
        }
    }
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
