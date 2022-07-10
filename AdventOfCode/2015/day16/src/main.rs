use std::{collections::HashMap, fs, process};

use phf::{phf_map, Map};
use regex::{Captures, Regex};
enum SIGN {
    GreaterThen,
    LessThen,
}

const MFCSAM_MSG: Map<&'static str, u32> = phf_map! (
    "children"=> 3,
    "cats"=> 7,
    "samoyeds"=> 2,
    "pomeranians"=> 3,
    "akitas"=> 0,
    "vizslas"=> 0,
    "goldfish"=> 5,
    "trees"=> 3,
    "cars"=> 2,
    "perfumes"=> 1
); // My First Crime Scene Analysis Machine

const DONT_CHECK_KEY: Map<&str, SIGN> = phf_map! ("cats" => SIGN::GreaterThen, "trees"=> SIGN::GreaterThen, "pomeranians"=> SIGN::LessThen, "goldfish"=> SIGN::LessThen);

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
    let data: Vec<HashMap<&str, u32>> = prepare_data(input);
    'first: for (index, sue) in data.iter().enumerate() {
        for (key, value) in sue {
            if *MFCSAM_MSG.get(key).unwrap() != *value {
                continue 'first;
            }
        }
        return index + 1;
    }
    0
}

fn solve_part_two(input: &str) -> usize {
    let data: Vec<HashMap<&str, u32>> = prepare_data(input);
    let _real_sues: u32 = 0;
    'first: for (index, sue) in data.iter().enumerate() {
        for (key, value) in sue {
            if let Some(range) = DONT_CHECK_KEY.get(key) {
                match range {
                    SIGN::GreaterThen => {
                        if *value <= *MFCSAM_MSG.get(key).unwrap() {
                            continue 'first;
                        }
                    }
                    SIGN::LessThen => {
                        if *value >= *MFCSAM_MSG.get(key).unwrap() {
                            continue 'first;
                        }
                    }
                }
            } else if *MFCSAM_MSG.get(key).unwrap() != *value {
                continue 'first;
            }
        }
        return index + 1;
    }
    0
}

fn prepare_data(input: &str) -> Vec<HashMap<&str, u32>> {
    let re: Regex = Regex::new(r"Sue \d+: (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
    let mut data: Vec<HashMap<&str, u32>> = Vec::new();
    for line in input.lines() {
        let captures: Captures = re.captures(line).unwrap();
        let mut dictionary: HashMap<&str, u32> = HashMap::new();
        dictionary.insert(
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        );
        dictionary.insert(
            captures.get(3).unwrap().as_str(),
            captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        );
        dictionary.insert(
            captures.get(5).unwrap().as_str(),
            captures.get(6).unwrap().as_str().parse::<u32>().unwrap(),
        );
        data.push(dictionary);
    }
    data
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
