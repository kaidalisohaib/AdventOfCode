use cipher_crypt::{Caesar, Cipher};
use regex::{Captures, Regex};
use std::{cmp::Ordering, collections::HashMap, fs, process};
fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let re: Regex = Regex::new(r"^([a-z]+)(\d+)\[([a-z]+)\]").unwrap();

    for line in input.lines() {
        let room: String = line
            .chars()
            .map(|x| x.to_string())
            .filter(|x| *x != "-")
            .collect::<Vec<String>>()
            .join("");
        let captures: Captures = re.captures(&room).unwrap();

        if is_real_room(&room) {
            sum += captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        }
    }
    sum
}

fn solve_part_two(input: &str) -> u32 {
    let mut real_rooms_indexs: Vec<usize> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let room: String = line
            .chars()
            .map(|x| x.to_string())
            .filter(|x| *x != "-")
            .collect::<Vec<String>>()
            .join("");

        if is_real_room(&room) {
            real_rooms_indexs.push(index);
        }
    }
    let all_rooms: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let re: Regex = Regex::new(r"^([a-z]+)(\d+)\[([a-z]+)\]").unwrap();
    for real_room_index in real_rooms_indexs {
        let room_without_dashes: String = all_rooms
            .get(real_room_index)
            .unwrap()
            .chars()
            .filter(|x| *x != '-')
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        let captures: Captures = re.captures(room_without_dashes.as_str()).unwrap();
        let c = Caesar::new(
            (captures.get(2).unwrap().as_str().parse::<u32>().unwrap() % 26)
                .try_into()
                .unwrap(),
        );
        let mut decrypted_msg: String = c.encrypt(all_rooms.get(real_room_index).unwrap()).unwrap();
        decrypted_msg = decrypted_msg.replace('-', " ");
        if decrypted_msg.contains("north") {
            return captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        }
    }
    0
}
fn is_real_room(room: &str) -> bool {
    let re: Regex = Regex::new(r"^([a-z]+)(\d+)\[([a-z]+)\]").unwrap();

    let captures: Captures = re.captures(room).unwrap();
    let mut dictionnary_character_numoftime: HashMap<char, u8> = HashMap::new();
    for character in captures.get(1).unwrap().as_str().chars() {
        if let Some(number_of_time) = dictionnary_character_numoftime.get_mut(&character) {
            *number_of_time += 1;
        } else {
            dictionnary_character_numoftime.insert(character, 1);
        }
    }
    let mut sorted_dic: Vec<(char, u8)> = dictionnary_character_numoftime
        .iter()
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<(char, u8)>>();
    sorted_dic.sort_by(|&a, &b| match a.1.cmp(&b.1) {
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => match char::cmp(&a.0, &b.0) {
            Ordering::Less => Ordering::Less,
            _ => Ordering::Greater,
        },
        Ordering::Less => Ordering::Greater,
    });
    let sorted_dic: Vec<char> = sorted_dic.iter().map(|&(x, _)| x).collect();
    let characters_order_to_verify: Vec<char> = captures
        .get(3)
        .unwrap()
        .as_str()
        .chars()
        .collect::<Vec<char>>();
    return characters_order_to_verify
        .iter()
        .zip(sorted_dic.iter())
        .all(|(&a, &b)| a == b);
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
