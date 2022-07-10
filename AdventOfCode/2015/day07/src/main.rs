use regex::{Captures, Regex};
use std::{collections::HashMap, fs, process};

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);

    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> i32 {
    let mut variables_equations: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut already_calculated_variables: HashMap<&str, i32> = HashMap::new();
    let mut operations: Vec<&str> = Vec::new();

    //Prepare the equations related to each variables

    println!("Preparing variables equations ...");
    for line in input.lines() {
        let regex: Regex = Regex::new(r"^(\w+) (?:(\w+) )?(?:(\w+) )?-> (\w+)$").unwrap();
        let captures: Captures = regex.captures(line).unwrap();

        let mut equation: Vec<&str> = Vec::new();
        for i in 1..4 {
            if let Some(capture) = captures.get(i) {
                equation.push(capture.as_str());
            }
        }
        variables_equations.insert(captures.get(4).unwrap().as_str(), equation);
    }
    println!("... Finished preparing variables equations");
    calculate_variable(
        "a",
        &variables_equations,
        &mut already_calculated_variables,
        &mut operations,
    )
}

fn solve_part_two(input: &str) -> i32 {
    let mut variables_equations: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut already_calculated_variables: HashMap<&str, i32> = HashMap::new();
    let mut operations: Vec<&str> = Vec::new();

    //Prepare the equations related to each variables
    println!("Preparing variables equations ...");
    for line in input.lines() {
        let regex: Regex = Regex::new(r"^(\w+) (?:(\w+) )?(?:(\w+) )?-> (\w+)$").unwrap();
        let captures: Captures = regex.captures(line).unwrap();

        let mut equation: Vec<&str> = Vec::new();
        for i in 1..4 {
            if let Some(capture) = captures.get(i) {
                equation.push(capture.as_str());
            }
        }
        variables_equations.insert(captures.get(4).unwrap().as_str(), equation);
    }
    println!("... Finished preparing variables equations");
    let first_variable_a: i32 = calculate_variable(
        "a",
        &variables_equations,
        &mut already_calculated_variables,
        &mut operations,
    );
    already_calculated_variables.clear();
    let first_variable_a: String = first_variable_a.to_string();
    *variables_equations.get_mut("b").unwrap() = vec![first_variable_a.as_str()];
    calculate_variable(
        "a",
        &variables_equations,
        &mut already_calculated_variables,
        &mut operations,
    )
}

#[allow(clippy::unnecessary_unwrap)]
// I could have used recursion but I didnt want to because it's dangerous
fn calculate_variable<'a>(
    variable: &'a str,
    variables_equations: &HashMap<&'a str, Vec<&'a str>>,
    already_calculated_variables: &mut HashMap<&'a str, i32>,
    operations: &mut Vec<&'a str>,
) -> i32 {
    operations.push(variable);
    // For as long as we dont find the value of the variable we keep looping
    while !already_calculated_variables.contains_key(variable) {
        let equation: &Vec<&str> = variables_equations.get(operations.last().unwrap()).unwrap();
        // Separate each case so it's easier to work with
        match equation.len() {
            // If the equation contain 1 parameter, it's a number or
            // a variable that is already calculated or need to be calculated
            1 => {
                if already_calculated_variables
                    .get(equation.get(0).unwrap())
                    .is_some()
                {
                    already_calculated_variables.insert(
                        operations.last().unwrap(),
                        *already_calculated_variables
                            .get(equation.get(0).unwrap())
                            .unwrap(),
                    );
                    operations.pop();
                } else if let Ok(value) = equation.get(0).unwrap().parse::<i32>() {
                    already_calculated_variables.insert(operations.last().unwrap(), value);
                    operations.pop();
                } else {
                    operations.push(equation.get(0).unwrap());
                }
            }
            // If the equation contain 2 parameter, then there is only one possible function the "NOT"
            2 => {
                if already_calculated_variables
                    .get(equation.get(1).unwrap())
                    .is_some()
                {
                    already_calculated_variables.insert(
                        operations.last().unwrap(),
                        bitwise_not(
                            *already_calculated_variables
                                .get(equation.get(1).unwrap())
                                .unwrap(),
                        ),
                    );
                    operations.pop();
                } else if let Ok(value) = equation.get(1).unwrap().parse::<i32>() {
                    already_calculated_variables
                        .insert(operations.last().unwrap(), bitwise_not(value));
                    operations.pop();
                } else {
                    operations.push(equation.get(1).unwrap());
                }
            }
            // If the equation contain 3 parameter, then we need to verify that the two numbers
            // are known and then we choose between the bitwise operators
            3 => {
                let first_param: Option<i32> = {
                    if let Ok(value) = equation.get(0).unwrap().parse::<i32>() {
                        Some(value)
                    } else if already_calculated_variables
                        .get(equation.get(0).unwrap())
                        .is_some()
                    {
                        Some(
                            *already_calculated_variables
                                .get(equation.get(0).unwrap())
                                .unwrap(),
                        )
                    } else {
                        operations.push(equation.get(0).unwrap());
                        None
                    }
                };
                let second_param: Option<i32> = {
                    if let Ok(value) = equation.get(2).unwrap().parse::<i32>() {
                        Some(value)
                    } else if already_calculated_variables
                        .get(equation.get(2).unwrap())
                        .is_some()
                    {
                        Some(
                            *already_calculated_variables
                                .get(equation.get(2).unwrap())
                                .unwrap(),
                        )
                    } else {
                        operations.push(equation.get(2).unwrap());
                        None
                    }
                };
                if first_param.is_some() && second_param.is_some() {
                    let first_param: i32 = first_param.unwrap();
                    let second_param: i32 = second_param.unwrap();
                    let mut calculated_value: i32 = 0;
                    match *equation.get(1).unwrap() {
                        "OR" => calculated_value = bitwise_or(first_param, second_param),
                        "AND" => calculated_value = bitwise_and(first_param, second_param),
                        "RSHIFT" => calculated_value = bitwise_rshift(first_param, second_param),
                        "LSHIFT" => calculated_value = bitwise_lshift(first_param, second_param),
                        _ => {}
                    }
                    already_calculated_variables
                        .insert(operations.last().unwrap(), calculated_value);
                    operations.pop();
                }
            }
            _ => {}
        }
    }
    *already_calculated_variables.get(variable).unwrap()
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

fn bitwise_or(number_one: i32, number_two: i32) -> i32 {
    number_one | number_two
}

fn bitwise_and(number_one: i32, number_two: i32) -> i32 {
    number_one & number_two
}

fn bitwise_not(number: i32) -> i32 {
    !number
}

fn bitwise_rshift(number_one: i32, number_two: i32) -> i32 {
    number_one >> number_two
}

fn bitwise_lshift(number_one: i32, number_two: i32) -> i32 {
    number_one << number_two
}
