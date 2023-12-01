// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path
};

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() {
    println!(
        "Sum of all calibration values with numerics parsed only: {}",
        sum_of_calibration_values(Path::new("./input/aoc_23_01/input.txt"), &parse_calibration_value_numeric)
    );
    println!(
        "Sum of all calibration values with numerics and written numbers parsed: {}",
        sum_of_calibration_values(Path::new("./input/aoc_23_01/input.txt"), &parse_calibration_value_anyformat)
    );
}


pub fn parse_calibration_value_numeric(in_str: String) -> usize {
    let c_front = in_str
        .chars()
        .filter(|c| c.is_numeric()).next();

    let c_back = in_str
        .chars()
        .filter(|c| c.is_numeric()).next_back();

    vec![ c_front, c_back ]
        .iter()
        .map(|option| option.unwrap())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}



pub fn parse_calibration_value_anyformat(in_str: String) -> usize {

    let numbers: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    let enumerated_numbers = numbers.into_iter().enumerate();
    let find_func = str::find::<&str>;

    let front = enumerated_numbers
        .map(|(index, num_str)| (index, find_func(in_str.as_str(), num_str)))
        .filter(|tuple| tuple.1.is_some())
        .map(|(index, find_index_option)| (index, find_index_option.unwrap()))
        .min_by(|(_, find_index1), (_, find_index2)| find_index1.cmp(find_index2))
        .map(|(index, _)| index % 10)
        .unwrap();

    let enumerated_numbers = numbers.into_iter().enumerate();
    let find_func = str::rfind::<&str>;

    let back = enumerated_numbers
        .map(|(index, num_str)| (index, find_func(in_str.as_str(), num_str)))
        .filter(|tuple| tuple.1.is_some())
        .map(|(index, find_index_option)| (index, find_index_option.unwrap()))
        .max_by(|(_, find_index1), (_, find_index2)| find_index1.cmp(find_index2))
        .map(|(index, _)| index % 10)
        .unwrap();

    front * 10 + back
}


pub fn sum_of_calibration_values(in_path: &Path, line_parse_strat: &dyn Fn(String) -> usize) -> usize {
    BufReader::new(File::open(in_path).expect("Input file does not open"))
        .lines()
        .map(|result| result.unwrap())
        .map(line_parse_strat)
        .sum()
}