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

fn solve_part_one(input: &str) -> u128 {
    let mut packages_weight: Vec<u32> = prepare_data(input);
    packages_weight.reverse();
    let weight_per_group: u32 = packages_weight.iter().sum::<u32>() / 3;
    let mut group1: Vec<u32> = packages_weight.clone();

    recursive_combination_group1(
        0,
        weight_per_group,
        Vec::new(),
        &packages_weight,
        &mut group1,
    );

    calculate_quantum_entanglement(&group1)
}

fn solve_part_two(input: &str) -> u128 {
    let mut packages_weight: Vec<u32> = prepare_data(input);
    packages_weight.reverse();
    let weight_per_group: u32 = packages_weight.iter().sum::<u32>() / 4;
    let mut group1: Vec<u32> = packages_weight.clone();

    recursive_combination_group1(
        0,
        weight_per_group,
        Vec::new(),
        &packages_weight,
        &mut group1,
    );

    calculate_quantum_entanglement(&group1)
}

fn recursive_combination_group1(
    next_start_index: usize,
    target: u32,
    list: Vec<u32>,
    packages_weight: &Vec<u32>,
    current_group1: &mut Vec<u32>,
) {
    for index in next_start_index..packages_weight.len() {
        let current_weight: u32 = list.iter().sum();
        if packages_weight[index] + current_weight > target {
            continue;
        }
        let mut cloned_list: Vec<u32> = list.clone();
        cloned_list.push(packages_weight[index]);
        if cloned_list.len() > current_group1.len() {
            continue;
        }
        if cloned_list.iter().sum::<u32>() == target {
            if cloned_list.len() < current_group1.len()
                || calculate_quantum_entanglement(&cloned_list)
                    < calculate_quantum_entanglement(current_group1)
            {
                *current_group1 = cloned_list;
            }
            continue;
        }
        recursive_combination_group1(
            index + 1,
            target,
            cloned_list,
            packages_weight,
            current_group1,
        );
    }
}

fn calculate_quantum_entanglement(group: &Vec<u32>) -> u128 {
    let mut value: u128 = 1;
    for package in group {
        value *= *package as u128;
    }
    value
}

fn prepare_data(input: &str) -> Vec<u32> {
    let mut list_of_weights: Vec<u32> = Vec::new();
    for line in input.lines() {
        list_of_weights.push(line.parse().unwrap());
    }
    list_of_weights
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
