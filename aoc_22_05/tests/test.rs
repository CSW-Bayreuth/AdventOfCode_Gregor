// ----------------------------------------------------
// imports
// ----------------------------------------------------
// use std::path::Path;

use aoc_22_05::*;

// ----------------------------------------------------
// tests constants
// ----------------------------------------------------
// const INPUT_PROPER_PATH: &str = "./../input/aoc_22_05/input.txt";
// const INPUT_SAMPLE_PATH: &str = "./../input/aoc_22_05/input_example.txt";

// ----------------------------------------------------
// test funcs part 1
// ----------------------------------------------------
#[test]
fn test_read_cargo() {
    assert_eq!(String::from("[A]").read(), Some('A'));
}

#[test]
fn test_read_crane_cmd() {
    assert_eq!(String::from("move 1 from 2 to 1").read(), Some((1, 2, 1)));
}
