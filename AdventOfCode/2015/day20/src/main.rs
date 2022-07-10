use std::{fs, process};

const N: u32 = 34_000_000;
fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("========================================");
    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(_input: &str) -> usize {
    let mut houses_presents: Vec<u32> = vec![0; (N / 10) as usize];
    let mut index1: u32 = 1;
    let end1: u32 = N / 10;
    'first: loop {
        let mut index2: u32 = index1;
        let end2: u32 = N / 10;
        'second: loop {
            houses_presents[(index2 - 1) as usize] += index1 * 10;
            /////////////
            index2 += index1;
            if index2 > end2 {
                break 'second;
            }
        }
        /////////////
        index1 += 1;
        if index1 > end1 {
            break 'first;
        }
    }
    for (index, house_presents) in houses_presents.iter().enumerate() {
        if *house_presents >= N {
            return index + 1;
        }
    }
    0
}

fn solve_part_two(_input: &str) -> usize {
    let mut houses_presents: Vec<u32> = vec![0; (N / 10) as usize];
    let mut index1: u32 = 1;
    let end1: u32 = N / 10;
    'first: loop {
        let mut index2: u32 = index1;
        let end2: u32 = N / 10;
        let mut number_of_delivery: u32 = 0;
        'second: loop {
            houses_presents[(index2 - 1) as usize] += index1 * 11;
            number_of_delivery += 1;
            if number_of_delivery >= 50 {
                break 'second;
            }
            /////////////
            index2 += index1;
            if index2 > end2 {
                break 'second;
            }
        }
        /////////////
        index1 += 1;
        if index1 > end1 {
            break 'first;
        }
    }
    for (index, house_presents) in houses_presents.iter().enumerate() {
        if *house_presents >= N {
            return index + 1;
        }
    }
    0
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
