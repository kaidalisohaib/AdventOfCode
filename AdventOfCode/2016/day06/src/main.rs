use std::{collections::HashMap, fs, process};

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
    let word_size: usize = input.lines().next().unwrap().trim().len();
    let mut character_repetitions: Vec<HashMap<char, u32>> = vec![HashMap::new(); word_size];
    for line in input.lines() {
        for (index, character) in line.char_indices() {
            let letter_dictionary: &mut HashMap<char, u32> =
                character_repetitions.get_mut(index).unwrap();
            if let Some(number_of_time) = letter_dictionary.get_mut(&character) {
                *number_of_time += 1;
            } else {
                letter_dictionary.insert(character, 1);
            }
        }
    }
    let mut word: String = String::new();
    for most_repeated_character_dict in character_repetitions {
        let mut list_of_repeated_character: Vec<(char, u32)> = most_repeated_character_dict
            .iter()
            .map(|(&x, &y)| (x, y))
            .collect::<Vec<(char, u32)>>();

        list_of_repeated_character.sort_by(|a, b| match u32::cmp(&a.1, &b.1) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
        });
        word.push(list_of_repeated_character.get(0).unwrap().0);
    }
    word
}

fn solve_part_two(input: &str) -> String {
    let word_size: usize = input.lines().next().unwrap().trim().len();
    let mut character_repetitions: Vec<HashMap<char, u32>> = vec![HashMap::new(); word_size];
    for line in input.lines() {
        for (index, character) in line.char_indices() {
            let letter_dictionary: &mut HashMap<char, u32> =
                character_repetitions.get_mut(index).unwrap();
            if let Some(number_of_time) = letter_dictionary.get_mut(&character) {
                *number_of_time += 1;
            } else {
                letter_dictionary.insert(character, 1);
            }
        }
    }
    let mut word: String = String::new();
    for most_repeated_character_dict in character_repetitions {
        let mut list_of_repeated_character: Vec<(char, u32)> = most_repeated_character_dict
            .iter()
            .map(|(&x, &y)| (x, y))
            .collect::<Vec<(char, u32)>>();

        list_of_repeated_character.sort_by(|a, b| u32::cmp(&a.1, &b.1));
        word.push(list_of_repeated_character.get(0).unwrap().0);
    }
    word
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
