// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;
use std::path::Path;

use aoc_22_06::*;

// ----------------------------------------------------
// tests constants
// ----------------------------------------------------
const INPUT_SAMPLE1_PATH: &str = "./../input/aoc_22_06/input_example1.txt";
const INPUT_SAMPLE2_PATH: &str = "./../input/aoc_22_06/input_example2.txt";
const INPUT_SAMPLE3_PATH: &str = "./../input/aoc_22_06/input_example3.txt";
const INPUT_SAMPLE4_PATH: &str = "./../input/aoc_22_06/input_example4.txt";
const INPUT_SAMPLE5_PATH: &str = "./../input/aoc_22_06/input_example5.txt";

// ----------------------------------------------------
// test funcs
// ----------------------------------------------------
#[rstest]
#[case(INPUT_SAMPLE1_PATH, 19)]
#[case(INPUT_SAMPLE2_PATH, 23)]
#[case(INPUT_SAMPLE3_PATH, 23)]
#[case(INPUT_SAMPLE4_PATH, 29)]
#[case(INPUT_SAMPLE5_PATH, 26)]
fn test_calc_som_marker_pos(#[case] input_path: &str, #[case] expected: usize) {
    assert_eq!(
        calc_som_marker_pos(read_input(Path::new(input_path))),
        expected
    )
}

#[rstest]
#[case(INPUT_SAMPLE1_PATH, 7)]
#[case(INPUT_SAMPLE2_PATH, 5)]
#[case(INPUT_SAMPLE3_PATH, 6)]
#[case(INPUT_SAMPLE4_PATH, 10)]
#[case(INPUT_SAMPLE5_PATH, 11)]
fn test_calc_sop_marker_pos(#[case] input_path: &str, #[case] expected: usize) {
    assert_eq!(
        calc_sop_marker_pos(read_input(Path::new(input_path))),
        expected
    )
}
