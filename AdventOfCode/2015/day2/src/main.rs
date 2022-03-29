use std::{fs, process};

fn main() {
    let file_content: String = read_input_file();
    println!("Input:\n{}", file_content);

    let part_one_answer = solve_part_one(&file_content);
    println!("Part one answer: {:?}", part_one_answer);
    let part_two_answer = solve_part_two(&file_content);
    println!("Part two answer: {:?}", part_two_answer);
}

fn solve_part_one(input: &str) -> u32 {
    let mut total_area: u32 = 0;
    for line in input.lines() {
        let l_w_h: Vec<u32> = line
            .split('x')
            .collect::<Vec<&str>>()
            .iter()
            .map(|number_str| number_str.parse::<u32>().unwrap())
            .collect();
        let area_lw: u32 = l_w_h[0] * l_w_h[1];
        let mut smallest_area: u32 = area_lw;
        let area_lh: u32 = l_w_h[0] * l_w_h[2];
        if smallest_area > area_lh {
            smallest_area = area_lh;
        }
        let area_wh: u32 = l_w_h[1] * l_w_h[2];
        if smallest_area > area_wh {
            smallest_area = area_wh;
        }
        total_area += area_lw * 2 + area_lh * 2 + area_wh * 2 + smallest_area;
    }
    return total_area;
}

fn solve_part_two(input: &str) -> u32 {
    let mut total_feet: u32 = 0;
    for line in input.lines() {
        let l_w_h: Vec<u32> = line
            .split('x')
            .collect::<Vec<&str>>()
            .iter()
            .map(|number_str| number_str.parse::<u32>().unwrap())
            .collect();
        let perimeter_lw: u32 = l_w_h[0] + l_w_h[0] + l_w_h[1] + l_w_h[1];
        let mut smallest_perimeter: u32 = perimeter_lw;
        let perimeter_lh: u32 = l_w_h[0] + l_w_h[0] + l_w_h[2] + l_w_h[2];
        if smallest_perimeter > perimeter_lh {
            smallest_perimeter = perimeter_lh;
        }
        let perimeter_wh: u32 = l_w_h[1] + l_w_h[1] + l_w_h[2] + l_w_h[2];
        if smallest_perimeter > perimeter_wh {
            smallest_perimeter = perimeter_wh;
        }
        total_feet += smallest_perimeter + l_w_h[0] * l_w_h[1] * l_w_h[2];
    }
    return total_feet;
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
