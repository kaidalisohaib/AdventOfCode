use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fs, process,
};

#[derive(Debug)]
struct Path {
    distance: u32,
    visited_id: Vec<usize>,
}

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);
    println!("===============================");
    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> u32 {
    let complete_paths: Vec<Path> = find_all_paths(input);
    let mut shortest_distance = complete_paths.first().unwrap().distance;
    for path in complete_paths  {
        if path.distance < shortest_distance{
            shortest_distance = path.distance;
        }
    }
    shortest_distance
}

fn solve_part_two(input: &str) -> u32 {
    let complete_paths: Vec<Path> = find_all_paths(input);
    let mut longest_distance = complete_paths.first().unwrap().distance;
    for path in complete_paths  {
        if path.distance > longest_distance{
            longest_distance = path.distance;
        }
    }
    longest_distance}

fn find_all_paths(input: &str) -> Vec<Path> {
    let re: Regex = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();
    let mut all_cities_id: HashMap<&str, usize> = HashMap::new();
    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
    // Preparing data for search
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let (first_city_name, second_city_name) = (
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str(),
        );
        if !all_cities_id.contains_key(first_city_name) {
            all_cities_id.insert(first_city_name, all_cities_id.len() + 1);
        }
        if !all_cities_id.contains_key(second_city_name) {
            all_cities_id.insert(second_city_name, all_cities_id.len() + 1);
        }
        distances.insert(
            (
                *all_cities_id.get(first_city_name).unwrap(),
                *all_cities_id.get(second_city_name).unwrap(),
            ),
            captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
        );
    }

    // Algorithm to find the solution
    let mut currently_searching_paths: VecDeque<Path> = VecDeque::new();
    let mut finished_path: Vec<Path> = Vec::new();
    for id in all_cities_id.values() {
        currently_searching_paths.push_back(Path {
            distance: 0,
            visited_id: vec![*id],
        })
    }
    // It's like a DFS but the algorithm have to visit all cities
    while currently_searching_paths.len() > 0 {
        let current_path: Path = currently_searching_paths.pop_front().unwrap();
        if current_path.visited_id.len() == all_cities_id.len() {
            finished_path.push(current_path);
            continue;
        }
        let neighbours: Vec<usize> =
            get_neighbours(*current_path.visited_id.last().unwrap(), &distances);
        for neighbour in neighbours {
            if !current_path.visited_id.contains(&neighbour) {
                currently_searching_paths.push_back(Path {
                    distance: current_path.distance
                        + get_distance(
                            *current_path.visited_id.last().unwrap(),
                            neighbour,
                            &distances,
                        ),
                    visited_id: {
                        let mut new_visited_id: Vec<usize> = current_path.visited_id.clone();
                        new_visited_id.push(neighbour);
                        new_visited_id
                    },
                })
            }
        }
    }
    finished_path
}
fn get_distance(
    first_city_id: usize,
    second_city_id: usize,
    distances: &HashMap<(usize, usize), u32>,
) -> u32 {
    if let Some(distance) = distances.get(&(first_city_id, second_city_id)) {
        return *distance;
    }
    if let Some(distance) = distances.get(&(second_city_id, first_city_id)) {
        return *distance;
    }
    return *distances.get(&(first_city_id, second_city_id)).unwrap();
}

fn get_neighbours(center: usize, distances: &HashMap<(usize, usize), u32>) -> Vec<usize> {
    let mut neighbours: Vec<usize> = Vec::new();
    for (first_city_id, second_city_id) in distances.keys() {
        if *first_city_id == center {
            neighbours.push(*second_city_id);
        } else if *second_city_id == center {
            neighbours.push(*first_city_id);
        }
    }
    return neighbours;
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
