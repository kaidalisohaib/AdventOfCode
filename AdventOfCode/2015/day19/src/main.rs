use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, process,
};

use regex::{Captures, Regex};
use priority_queue::PriorityQueue;

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
    let (replacements, initial_molecule): (Vec<(&str, &str)>, String) = prepare_data(input);
    let mut all_possibilities: HashSet<String> = HashSet::new();
    for (atom, atom_replacement) in replacements {
        for (index, matches) in initial_molecule.match_indices(atom) {
            let mut cloned_initial_molecule: String = initial_molecule.clone();
            cloned_initial_molecule.replace_range(index..(index + atom.len()), atom_replacement);
            all_possibilities.insert(cloned_initial_molecule);
        }
    }
    all_possibilities.len()
}

fn solve_part_two(input: &str) -> u32 {
    let (replacements, medicine_molecule): (Vec<(&str, &str)>, String) = prepare_data(input);
    let goal_molecule: String = String::from("e");
    let mut all_possibilities: HashSet<String> = HashSet::new();
    let mut next_tries: PriorityQueue<(String, u32),i32> = PriorityQueue::new();
    next_tries.push((medicine_molecule.clone(), 0), -(medicine_molecule.len()as i32));
    loop {
        let (next_try, current_step): (String, u32) = next_tries.pop().unwrap().0;
        // I am going to do the inverse to shrink the size of the medicine molecule to "e"
        for (atom, atom_replacement) in &replacements {
            for (index, _) in next_try.match_indices(*atom_replacement) {
                let mut cloned_initial_molecule: String = next_try.clone();
                cloned_initial_molecule.replace_range(index..(index + (*atom_replacement).len()), *atom);
                if cloned_initial_molecule == goal_molecule {
                    return current_step + 1;
                } else if !all_possibilities.contains(&cloned_initial_molecule) {
                    next_tries.push((cloned_initial_molecule.clone(), current_step + 1), -(cloned_initial_molecule.len() as i32));
                }
                all_possibilities.insert(cloned_initial_molecule);
            }
        }
    }
}

fn prepare_data(input: &str) -> (Vec<(&str, &str)>, String) {
    let replacement_re: Regex = Regex::new(r"(\w+) => (\w+)").unwrap();
    let mut replacements: Vec<(&str, &str)> = Vec::new();
    let initial_molecule: String = String::from(input.lines().last().unwrap());
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let captures: Captures = replacement_re.captures(line).unwrap();
        let key: &str = captures.get(1).unwrap().as_str();
        let value: &str = captures.get(2).unwrap().as_str();
        replacements.push((key, value));
    }
    (replacements, initial_molecule)
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
