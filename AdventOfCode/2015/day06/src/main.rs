use regex::{Captures, Regex};
use std::{
    collections::{HashMap, HashSet},
    fs, process,
};

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);

    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> usize {
    let mut grid: HashSet<(u16, u16)> = HashSet::new();
    let reg: Regex =
        Regex::new(r"^(\w+) (?:(\w+) )?(\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})$").unwrap();
    for line in input.lines() {
        let groups: Captures = reg.captures(line).unwrap();
        let start: (u16, u16) = (
            groups.get(3).unwrap().as_str().parse().unwrap(),
            groups.get(4).unwrap().as_str().parse().unwrap(),
        );
        let end: (u16, u16) = (
            groups.get(5).unwrap().as_str().parse().unwrap(),
            groups.get(6).unwrap().as_str().parse().unwrap(),
        );
        match groups.get(1).unwrap().as_str() {
            "turn" => match groups.get(2).unwrap().as_str() {
                "on" => turn_on_part_one(&mut grid, start, end),
                "off" => turn_off_part_one(&mut grid, start, end),
                _ => {}
            },
            "toggle" => toggle_part_one(&mut grid, start, end),
            _ => {}
        }
    }
    grid.len()
}

fn turn_on_part_one(grid: &mut HashSet<(u16, u16)>, start: (u16, u16), end: (u16, u16)) {
    for index_x in start.0..=end.0 {
        for index_y in start.1..=end.1 {
            grid.insert((index_x, index_y));
        }
    }
}
fn turn_off_part_one(grid: &mut HashSet<(u16, u16)>, start: (u16, u16), end: (u16, u16)) {
    for index_x in start.0..=end.0 {
        for index_y in start.1..=end.1 {
            grid.remove(&(index_x, index_y));
        }
    }
}
fn toggle_part_one(grid: &mut HashSet<(u16, u16)>, start: (u16, u16), end: (u16, u16)) {
    for index_x in start.0..=end.0 {
        for index_y in start.1..=end.1 {
            if grid.contains(&(index_x, index_y)) {
                grid.remove(&(index_x, index_y));
            } else {
                grid.insert((index_x, index_y));
            }
        }
    }
}

fn solve_part_two(input: &str) -> u32 {
    let mut grid: HashMap<(u16, u16), u16> = HashMap::new();
    let reg: Regex =
        Regex::new(r"^(\w+) (?:(\w+) )?(\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})$").unwrap();
    for line in input.lines() {
        let groups: Captures = reg.captures(line).unwrap();
        let start: (u16, u16) = (
            groups.get(3).unwrap().as_str().parse().unwrap(),
            groups.get(4).unwrap().as_str().parse().unwrap(),
        );
        let end: (u16, u16) = (
            groups.get(5).unwrap().as_str().parse().unwrap(),
            groups.get(6).unwrap().as_str().parse().unwrap(),
        );
        match groups.get(1).unwrap().as_str() {
            "turn" => match groups.get(2).unwrap().as_str() {
                "on" => turn_on_part_two(&mut grid, start, end),
                "off" => turn_off_part_two(&mut grid, start, end),
                _ => {}
            },
            "toggle" => toggle_part_two(&mut grid, start, end),
            _ => {}
        }
    }
    let mut sum_val: u32 = 0;
    for value in grid.values() {
        sum_val += *value as u32;
    }
    sum_val
}

fn turn_on_part_two(grid: &mut HashMap<(u16, u16), u16>, start: (u16, u16), end: (u16, u16)) {
    for index_x in start.0..=end.0 {
        for index_y in start.1..=end.1 {
            if let Some(brightness) = grid.get_mut(&(index_x, index_y)) {
                *brightness += 1;
            } else {
                grid.insert((index_x, index_y), 1);
            }
        }
    }
}
fn turn_off_part_two(grid: &mut HashMap<(u16, u16), u16>, start: (u16, u16), end: (u16, u16)) {
    for index_x in start.0..=end.0 {
        for index_y in start.1..=end.1 {
            if let Some(brightness) = grid.get_mut(&(index_x, index_y)) {
                if *brightness == 1 {
                    grid.remove(&(index_x, index_y));
                } else {
                    *brightness -= 1;
                }
            }
        }
    }
}
fn toggle_part_two(grid: &mut HashMap<(u16, u16), u16>, start: (u16, u16), end: (u16, u16)) {
    for index_x in start.0..=end.0 {
        for index_y in start.1..=end.1 {
            if let Some(brightness) = grid.get_mut(&(index_x, index_y)) {
                *brightness += 2;
            } else {
                grid.insert((index_x, index_y), 2);
            }
        }
    }
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
