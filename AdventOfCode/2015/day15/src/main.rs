use std::{fs, process};

use regex::{Captures, Regex};

const TOTAL_TEASPOONS: u32 = 100;

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
    let ingredients: Vec<Vec<i32>> = prepare_data(input);

    let mut best_score: u32 = 0;
    recursive_combination(
        ingredients.len() as u32,
        TOTAL_TEASPOONS,
        Vec::new(),
        &ingredients,
        &mut best_score,
        false,
    );
    best_score
}

fn solve_part_two(input: &str) -> u32 {
    let ingredients: Vec<Vec<i32>> = prepare_data(input);

    let mut best_score: u32 = 0;
    recursive_combination(
        ingredients.len() as u32,
        TOTAL_TEASPOONS,
        Vec::new(),
        &ingredients,
        &mut best_score,
        true,
    );
    best_score
}

fn recursive_combination(
    level: u32,
    target: u32,
    mut content_list: Vec<u32>,
    ingredients: &Vec<Vec<i32>>,
    best_score: &mut u32,
    part_two: bool,
) {
    if level == 1 {
        let content_sum: u32 = (&content_list).iter().sum();
        content_list.push(target - content_sum);
        let ingredients_teaspoons: Vec<(&Vec<i32>, &u32)> =
            ingredients.iter().zip(content_list.iter()).collect();
        let score: u32 = calculate_score(&ingredients_teaspoons, part_two);
        if score > *best_score {
            *best_score = score;
        }
    } else {
        let content_sum: u32 = (&content_list).iter().sum();
        for i in 0..=target - content_sum {
            let mut cloned_content_list = content_list.clone();
            cloned_content_list.push(i);
            recursive_combination(
                level - 1,
                target,
                cloned_content_list,
                ingredients,
                best_score,
                part_two,
            );
        }
    }
}

fn calculate_score(ingredients: &Vec<(&Vec<i32>, &u32)>, part_two: bool) -> u32 {
    let mut properties_sum: Vec<i32> = vec![0; ingredients[0].0.len()];
    for index in 0..properties_sum.len() {
        for (ingredient, teaspoons_number) in ingredients {
            properties_sum[index] += ingredient[index] * (**teaspoons_number as i32);
        }
        if properties_sum[index] < 0 {
            properties_sum[index] = 0;
        }
    }
    if part_two && *properties_sum.last().unwrap() != 500 {
        return 0;
    }
    let mut total_score: i32 = properties_sum[0];
    for index in properties_sum.iter().take(properties_sum.len() - 1).skip(1) {
        total_score *= properties_sum.get(*index as usize).unwrap();
    }
    total_score as u32
}

fn prepare_data(input: &str) -> Vec<Vec<i32>> {
    let mut ingredients: Vec<Vec<i32>> = Vec::new();

    let re: Regex = Regex::new(r"^\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$").unwrap();
    //preparing data
    for line in input.lines() {
        let captures: Captures = re.captures(line).unwrap();
        let (capacity_delta, durability_delta, flavor_delta, texture_delta, calories_delta): (
            i32,
            i32,
            i32,
            i32,
            i32,
        ) = (
            captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(5).unwrap().as_str().parse::<i32>().unwrap(),
        );
        ingredients.push(vec![
            capacity_delta,
            durability_delta,
            flavor_delta,
            texture_delta,
            calories_delta,
        ]);
    }
    ingredients
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
