// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::path::Path;

use aoc_22_03::*;

// ----------------------------------------------------
// tests constants
// ----------------------------------------------------
const INPUT_PROPER_PATH: &str = "./../input/aoc_22_03/input.txt";
const INPUT_SAMPLE_PATH: &str = "./../input/aoc_22_03/input_example.txt";

// ----------------------------------------------------
// test funcs
// ----------------------------------------------------
#[test]
fn test_read_inputproper_300_knapsacks() {
    assert_eq!(
        Path::new(INPUT_PROPER_PATH).read().len(),
        300,
        "aoc_22_03/input.txt has 300 entries, but a different number was parsed"
    );
}

#[test]
fn test_read_inputsample_6_knapsacks() {
    assert_eq!(
        Path::new(INPUT_SAMPLE_PATH).read().len(),
        6,
        "aoc_22_03/input.txt has 6 entries, but a different number was parsed"
    );
}

#[test]
fn test_read_same_sized_compartments() {
    let knapsack = String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").read();
    assert_eq!(
        knapsack.cp1.len(),
        knapsack.cp2.len(),
        "knapsack compartment sizes must be equal"
    )
}
