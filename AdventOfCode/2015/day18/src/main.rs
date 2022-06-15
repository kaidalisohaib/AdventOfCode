use std::{fs, process};

const ON_CHAR: char = '#';
const OFF_CHAR: char = '.';
const STEPS: u32 = 100;
const GRID_SIZE: usize = 100;
const NEIGHBOURS_OFFSET: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
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
    let mut grid: [[bool; GRID_SIZE]; GRID_SIZE] = prepare_data(input);
    for index in 0..STEPS {
        grid = next_grid(&grid);
    }
    return grid.iter().flatten().filter(|x| **x).count();
}

fn solve_part_two(input: &str) -> usize {
    let mut grid: [[bool; GRID_SIZE]; GRID_SIZE] = prepare_data(input);
    grid[0][0] = true;
    grid[0][GRID_SIZE - 1] = true;
    grid[GRID_SIZE - 1][0] = true;
    grid[GRID_SIZE - 1][GRID_SIZE - 1] = true;
    for index in 0..STEPS {
        grid = next_grid(&grid);
        grid[0][0] = true;
        grid[0][GRID_SIZE - 1] = true;
        grid[GRID_SIZE - 1][0] = true;
        grid[GRID_SIZE - 1][GRID_SIZE - 1] = true;
    }
    return grid.iter().flatten().filter(|x| **x).count();
}

fn next_grid(grid: &[[bool; GRID_SIZE]; GRID_SIZE]) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut new_grid: [[bool; GRID_SIZE]; GRID_SIZE] = (*grid).clone();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let number_of_neighbour = NEIGHBOURS_OFFSET
                .iter()
                .filter(|(y_offset, x_offset)| {
                    let y_index = y as i8 + *y_offset;
                    let x_index = x as i8 + *x_offset;
                    if y_index < 0
                        || x_index < 0
                        || y_index as usize >= grid.len()
                        || x_index as usize >= grid[y].len()
                    {
                        return false;
                    }
                    return *grid
                        .get(y_index as usize)
                        .unwrap()
                        .get(x_index as usize)
                        .unwrap();
                })
                .count();
            match grid[y][x] {
                true => {
                    if number_of_neighbour != 2 && number_of_neighbour != 3 {
                        new_grid[y][x] = false;
                    }
                }
                false => {
                    if number_of_neighbour == 3 {
                        new_grid[y][x] = true
                    }
                }
            }
        }
    }
    new_grid
}

fn prepare_data(input: &str) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut grid: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.char_indices() {
            match character {
                ON_CHAR => grid[y][x] = true,
                OFF_CHAR => grid[y][x] = false,
                _ => {
                    println!("{}", character);
                    panic!("A character shouldn't be here !")
                }
            }
        }
    }
    grid
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
