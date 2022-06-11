use std::{fs, process};

use regex::{Captures, Regex};

const TIME_TO_CALCULATE: u32 = 2503;
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
    let mut farthest_distance: u32 = 0;
    let all_reindeers: Vec<(u32, u32, u32)> = prepare_data(input);
    for (speed, movement_time, stop_time) in all_reindeers {
        let distance: u32 =
            calculate_distance((speed, movement_time, stop_time), TIME_TO_CALCULATE);
        if distance > farthest_distance {
            farthest_distance = distance;
        }
    }
    farthest_distance
}

fn solve_part_two(input: &str) -> u32 {
    let all_reindeers: Vec<(u32, u32, u32)> = prepare_data(input);
    let mut reindeers_points: Vec<u32> = vec![0; all_reindeers.len()];
    for current_time in 1..=TIME_TO_CALCULATE {
        let (mut farthest_distance, mut farthest_distance_index): (u32, usize) = (0, 0);
        for (index, (speed, movement_time, stop_time)) in (&all_reindeers).into_iter().enumerate() {
            let distance: u32 =
                calculate_distance((*speed, *movement_time, *stop_time), current_time);
            if distance > farthest_distance {
                farthest_distance = distance;
                farthest_distance_index = index;
            }
        }
        *reindeers_points.get_mut(farthest_distance_index).unwrap() += 1;
    }
    let mut highest_points: u32 = *reindeers_points.get(0).unwrap();
    for reindeer_points in reindeers_points {
        if reindeer_points > highest_points {
            highest_points = reindeer_points;
        }
    }
    highest_points
}

fn calculate_distance(
    (speed, movement_time, stop_time): (u32, u32, u32),
    time_to_calculate: u32,
) -> u32 {
    time_to_calculate / (movement_time + stop_time) * speed * movement_time + {
        let incomplete_time: u32 = time_to_calculate % (movement_time + stop_time);
        if incomplete_time >= movement_time {
            movement_time * speed
        } else {
            incomplete_time * speed
        }
    }
}

fn prepare_data(input: &str) -> Vec<(u32, u32, u32)> {
    let re: Regex = Regex::new(
        r"^\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$",
    )
    .unwrap();
    let mut all_reindeers: Vec<(u32, u32, u32)> = Vec::new();
    for line in input.lines() {
        let matchs: Captures = re.captures(line).unwrap();
        let (speed, movement_time, stop_time): (u32, u32, u32) = (
            matchs.get(1).unwrap().as_str().parse().unwrap(),
            matchs.get(2).unwrap().as_str().parse().unwrap(),
            matchs.get(3).unwrap().as_str().parse().unwrap(),
        );
        all_reindeers.push((speed, movement_time, stop_time));
    }
    all_reindeers
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
