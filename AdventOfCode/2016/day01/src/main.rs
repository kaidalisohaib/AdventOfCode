use std::{collections::HashSet, fs, process};

type Instruction = (bool, i32); // Turning left ? | Walk how many blocs

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
    let instructions: Vec<Instruction> = input
        .split(',')
        .map(|x| {
            let str_instruction: (&str, &str) = x.trim().split_at(1);
            let turning_left: bool = str_instruction.0 == "L";
            let walking_blocs: i32 = str_instruction.1.parse::<i32>().unwrap();
            (turning_left, walking_blocs)
        })
        .collect::<Vec<Instruction>>();
    let mut current_angle: i32 = 90;
    let mut current_position: (i32, i32) = (0, 0);

    for (turn_left, walk_blocs) in &instructions {
        current_angle = clamp_angle_between_0_360(if *turn_left {
            current_angle + 90
        } else {
            current_angle - 90
        });
        match current_angle {
            0 => current_position.0 += *walk_blocs,
            90 => current_position.1 += *walk_blocs,
            180 => current_position.0 -= *walk_blocs,
            270 => current_position.1 -= *walk_blocs,
            _ => panic!("Impossible angle given the situation !"),
        }
    }

    (current_position.0.abs() + current_position.1.abs()) as u32
}

fn solve_part_two(input: &str) -> u32 {
    let instructions: Vec<Instruction> = input
        .split(',')
        .map(|x| {
            let str_instruction: (&str, &str) = x.trim().split_at(1);
            let turning_left: bool = str_instruction.0 == "L";
            let walking_blocs: i32 = str_instruction.1.parse::<i32>().unwrap();
            (turning_left, walking_blocs)
        })
        .collect::<Vec<Instruction>>();
    let mut current_angle: i32 = 90;
    let mut current_position: (i32, i32) = (0, 0);
    let mut all_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    for (turn_left, walk_blocs) in &instructions {
        current_angle = clamp_angle_between_0_360(if *turn_left {
            current_angle + 90
        } else {
            current_angle - 90
        });
        for _step in 0..*walk_blocs {
            match current_angle {
                0 => current_position.0 += 1,
                90 => current_position.1 += 1,
                180 => current_position.0 -= 1,
                270 => current_position.1 -= 1,
                _ => panic!("Impossible angle given the situation !"),
            }

            if !all_positions.insert(current_position) {
                return (current_position.0.abs() + current_position.1.abs()) as u32;
            }
        }
    }
    0
}

fn clamp_angle_between_0_360(angle: i32) -> i32 {
    let new_angle: i32 = angle % 360;
    if new_angle < 0 {
        return new_angle + 360;
    }
    new_angle
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
