#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_01::*;

// ----------------------------------------------------
// tests funcs
// ----------------------------------------------------

#[rstest]
#[case("1abc2", 12)]
#[case("pqr3stu8vwx", 38)]
#[case("a1b2c3d4e5f", 15)]
#[case("treb7uchet", 77)]
fn test_parse_calibration_value_numeric(
    #[case] input_line: &str,
    #[case] expected: usize
) {
    assert_eq!(
        parse_calibration_value_numeric(input_line.to_string()),
        expected,
    );
}

#[rstest]
#[case("two1nine", 29)]
#[case("eightwothree", 83)]
#[case("abcone2threexyz", 13)]
#[case("xtwone3four", 24)]
#[case("4nineeightseven2", 42)]
#[case("zoneight234", 14)]
#[case("7pqrstsixteen", 76)]
fn test_parse_calibration_value_anyformat(
    #[case] input_line: &str,
    #[case] expected: usize
) {
    assert_eq!(
        parse_calibration_value_anyformat(input_line.to_string()),
        expected,
    );
}

#[rstest]
#[case("./../input/aoc_23_01/input_example.txt",  &parse_calibration_value_numeric, 142)]
#[case("./../input/aoc_23_01/input2_example.txt", &parse_calibration_value_anyformat, 281)]
fn test_sum_of_calibration_values_numeric(
    #[case] input_path: &str,
    #[case] line_parse_strat: &dyn Fn(String) -> usize,
    #[case] expected: usize
) {
    assert_eq!(
        sum_of_calibration_values(std::path::Path::new(input_path), line_parse_strat),
        expected,
    );
}
