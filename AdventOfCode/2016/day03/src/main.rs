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

fn solve_part_one(input: &str) -> u32 {
    let mut possible_triangle: u32 = 0;
    for line in input.lines() {
        let triangle_sides: Vec<u32> = line
            .trim()
            .split("  ")
            .filter(|x| !(*x).trim().is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if triangle_sides[0] + triangle_sides[1] <= triangle_sides[2] {
            continue;
        }
        if triangle_sides[0] + triangle_sides[2] <= triangle_sides[1] {
            continue;
        }
        if triangle_sides[1] + triangle_sides[2] <= triangle_sides[0] {
            continue;
        }
        possible_triangle += 1;
    }
    possible_triangle
}

fn solve_part_two(input: &str) -> u32 {
    let mut possible_triangle: u32 = 0;
    let mut row_numbers: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let row_triangle: Vec<u32> = line
            .trim()
            .split("  ")
            .filter(|x| !(*x).trim().is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        row_numbers.push(row_triangle);
    }
    for column in 0..3 {
        for group in 0..row_numbers.len() / 3 {
            let real_index: usize = group * 3;
            let first_side: u32 = row_numbers[real_index][column];
            let second_side: u32 = row_numbers[real_index + 1][column];
            let third_side: u32 = row_numbers[real_index + 2][column];
            if first_side + second_side <= third_side {
                continue;
            }
            if first_side + third_side <= second_side {
                continue;
            }
            if second_side + third_side <= first_side {
                continue;
            }
            possible_triangle += 1;
        }
    }

    possible_triangle
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
