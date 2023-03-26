// ----------------------------------------------------
// imports
// ----------------------------------------------------
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
#[test]
fn test_calc_som_marker_pos_sample1() {
    assert_eq!(
        calc_som_marker_pos(read_input(Path::new(INPUT_SAMPLE1_PATH))),
        19
    )
}
#[test]
fn test_calc_som_marker_pos_sample2() {
    assert_eq!(
        calc_som_marker_pos(read_input(Path::new(INPUT_SAMPLE2_PATH))),
        23
    )
}
#[test]
fn test_calc_som_marker_pos_sample3() {
    assert_eq!(
        calc_som_marker_pos(read_input(Path::new(INPUT_SAMPLE3_PATH))),
        23
    )
}
#[test]
fn test_calc_som_marker_pos_sample4() {
    assert_eq!(
        calc_som_marker_pos(read_input(Path::new(INPUT_SAMPLE4_PATH))),
        29
    )
}
#[test]
fn test_calc_som_marker_pos_sample5() {
    assert_eq!(
        calc_som_marker_pos(read_input(Path::new(INPUT_SAMPLE5_PATH))),
        26
    )
}

// ----------------------------------------------------
// test funcs
// ----------------------------------------------------
#[test]
fn test_calc_sop_marker_pos_sample1() {
    assert_eq!(
        calc_sop_marker_pos(read_input(Path::new(INPUT_SAMPLE1_PATH))),
        7
    )
}
#[test]
fn test_calc_sop_marker_pos_sample2() {
    assert_eq!(
        calc_sop_marker_pos(read_input(Path::new(INPUT_SAMPLE2_PATH))),
        5
    )
}
#[test]
fn test_calc_sop_marker_pos_sample3() {
    assert_eq!(
        calc_sop_marker_pos(read_input(Path::new(INPUT_SAMPLE3_PATH))),
        6
    )
}
#[test]
fn test_calc_sop_marker_pos_sample4() {
    assert_eq!(
        calc_sop_marker_pos(read_input(Path::new(INPUT_SAMPLE4_PATH))),
        10
    )
}
#[test]
fn test_calc_sop_marker_pos_sample5() {
    assert_eq!(
        calc_sop_marker_pos(read_input(Path::new(INPUT_SAMPLE5_PATH))),
        11
    )
}
