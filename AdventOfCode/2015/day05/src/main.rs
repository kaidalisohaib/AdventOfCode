use fancy_regex::Regex;
use std::{fs, process};

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);

    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> u16 {
    let mut number_nice: u16 = 0;
    let three_vowey: Regex = Regex::new(r"(?:.*[aeiou].*){3,}").unwrap();
    let black_list: Regex = Regex::new(r"^(?!.*(?:ab|cd|pq|xy)).*").unwrap();
    let twice_letter: Regex = Regex::new(r".*(\w)\1.*").unwrap();
    for line in input.lines() {
        let match_three_vowey = three_vowey.is_match(line).unwrap();
        if !match_three_vowey {
            continue;
        }
        let match_black_list = black_list.is_match(line).unwrap();
        if !match_black_list {
            continue;
        }
        let match_twice_letter = twice_letter.is_match(line).unwrap();
        if !match_twice_letter {
            continue;
        }
        number_nice += 1;
    }
    number_nice
}

fn solve_part_two(input: &str) -> u16 {
    let mut number_nice: u16 = 0;
    let one_letter: Regex = Regex::new(r".*(\w)\w\1.*").unwrap();
    let twice_letter: Regex = Regex::new(r".*(\w{2}).*\1.*").unwrap();
    for line in input.lines() {
        let match_three_vowey = one_letter.is_match(line).unwrap();
        if !match_three_vowey {
            continue;
        }
        let match_twice_letter = twice_letter.is_match(line).unwrap();
        if !match_twice_letter {
            continue;
        }
        number_nice += 1;
    }
    number_nice
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
