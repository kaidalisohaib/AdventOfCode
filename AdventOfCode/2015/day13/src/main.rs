use regex::{Captures, Regex};
use std::{collections::HashMap, fs, process, thread::sleep, time::Duration};

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
    let (people_ids, happiness_table): (HashMap<&str, usize>, HashMap<(usize, usize), i32>) =
        prepare_data(input);
    let mut all_ids: Vec<usize> = people_ids.values().map(|x| *x).collect();
    let mut highest_happiness_value: i32 = calculate_happiness(&all_ids, &happiness_table);
    recusive_permutation(
        all_ids.clone(),
        0,
        &happiness_table,
        &mut highest_happiness_value,
    );
    highest_happiness_value
}

fn solve_part_two(input: &str) -> i32{
    let (mut people_ids, mut happiness_table): (HashMap<&str, usize>, HashMap<(usize, usize), i32>) =
        prepare_data(input);
    let my_id = people_ids.len() + 1;
    for id in people_ids.values() {
        happiness_table.insert((my_id, *id), 0);
        happiness_table.insert((*id, my_id), 0);
    }
    people_ids.insert("Me", my_id);
    let mut all_ids: Vec<usize> = people_ids.values().map(|x| *x).collect();
    let mut highest_happiness_value: i32 = calculate_happiness(&all_ids, &happiness_table);
    recusive_permutation(
        all_ids.clone(),
        0,
        &happiness_table,
        &mut highest_happiness_value,
    );
    highest_happiness_value
}

fn recusive_permutation(
    mut all_ids: Vec<usize>,
    level: usize,
    happiness_table: &HashMap<(usize, usize), i32>,
    highest_happiness_value: &mut i32,
) {
    if level == all_ids.len() - 1 {
        let happiness_value = calculate_happiness(&all_ids, happiness_table);
        if happiness_value > *highest_happiness_value {
            *highest_happiness_value = happiness_value;
        }
    } else {
        let mut temp: usize = 0;
        for index in level..all_ids.len() {
            temp = all_ids[index];
            all_ids[index] = all_ids[level];
            all_ids[level] = temp;
            recusive_permutation(
                all_ids.clone(),
                level + 1,
                happiness_table,
                highest_happiness_value,
            );
        }
    }
}

fn calculate_happiness(places: &Vec<usize>, happiness_table: &HashMap<(usize, usize), i32>) -> i32 {
    let mut sum = 0;
    for (index, id) in places.iter().enumerate() {
        let left_index: usize = {
            let temp: i32 = index as i32 - 1;
            if temp < 0 {
                places.len() - 1 as usize
            } else {
                temp as usize
            }
        };
        let right_index: usize = {
            let temp: i32 = index as i32 + 1;
            if temp > places.len() as i32 - 1 {
                0
            } else {
                temp as usize
            }
        };
        sum += happiness_table.get(&(*id, places[left_index])).unwrap();
        sum += happiness_table.get(&(*id, places[right_index])).unwrap();
    }
    return sum;
}

fn prepare_data(input: &str) -> (HashMap<&str, usize>, HashMap<(usize, usize), i32>) {
    let re: Regex =
        Regex::new(r"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).$").unwrap();
    let mut people_ids: HashMap<&str, usize> = HashMap::new();
    let mut happiness_table: HashMap<(usize, usize), i32> = HashMap::new();
    // preparing data
    for line in input.lines() {
        let info: Captures = re.captures(line).unwrap();
        let first_name: &str = info.get(1).unwrap().as_str();
        let second_name: &str = info.get(4).unwrap().as_str();
        if !people_ids.contains_key(first_name) {
            people_ids.insert(first_name, people_ids.len() + 1);
        }
        if !people_ids.contains_key(second_name) {
            people_ids.insert(second_name, people_ids.len() + 1);
        }
        happiness_table.insert(
            (
                *people_ids.get(first_name).unwrap(),
                *people_ids.get(second_name).unwrap(),
            ),
            {
                let absolute_delta: i32 = info.get(3).unwrap().as_str().parse::<i32>().unwrap();
                match info.get(2).unwrap().as_str() {
                    "gain" => absolute_delta,
                    "lose" => -absolute_delta,
                    _ => panic!("Did not found \"gain\" or \"lose\"."),
                }
            },
        );
    }
    (people_ids, happiness_table)
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
