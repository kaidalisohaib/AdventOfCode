use std::{fs, process, collections::HashMap};

const VOLUME_OF_EGGNOG: u32 = 150;
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
    let containers: Vec<u32> = prepare_data(input);
    let mut all_possibilities: Vec<Vec<u32>> = Vec::new();
    recursive_combination(0, 0, VOLUME_OF_EGGNOG, &containers, Vec::new(), &mut all_possibilities);
    all_possibilities.len()
}

fn solve_part_two(input: &str) -> u32 {
    let containers: Vec<u32> = prepare_data(input);
    let mut all_possibilities: Vec<Vec<u32>> = Vec::new();
    recursive_combination(0, 0, VOLUME_OF_EGGNOG, &containers, Vec::new(), &mut all_possibilities);
    let mut sizes: HashMap<usize, u32> = HashMap::new();
    for container_combination in all_possibilities {
        if let Some(value) = sizes.get_mut(&container_combination.len()){
            *value += 1;
        }else{
            sizes.insert(container_combination.len(), 1);
        }
    }
    let (mut smallest_container_size, mut number_of_container) : (&usize, &u32) = sizes.iter().next().unwrap();
    for (container_size, value) in sizes.iter() {
        if container_size < smallest_container_size{
            smallest_container_size = container_size;
            number_of_container = value;
        }
    }
    *number_of_container
}

fn recursive_combination(start_index: usize, current_sum: u32, target: u32, containers: &Vec<u32>, content_vector: Vec<u32>, all_possibilities: &mut Vec<Vec<u32>>){
    for index in start_index..containers.len() {
        let sum: u32 = current_sum + containers[index];
        if sum > target{
            continue;
        }
        let mut cloned_vector: Vec<u32> = content_vector.clone();
        cloned_vector.push(containers[index]);
        if sum == target{
            all_possibilities.push(cloned_vector);
            continue;
        }
        recursive_combination(index + 1, sum, target, containers, cloned_vector, all_possibilities);
        
    }
}

fn prepare_data(input: &str) -> Vec<u32> {
    let mut containers: Vec<u32> = Vec::new();
    for line in input.lines() {
        containers.push(line.parse::<u32>().unwrap());
    }
    containers
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
