// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::path::Path;

use aoc_22_04::*;

// ----------------------------------------------------
// tests constants
// ----------------------------------------------------
const INPUT_PROPER_PATH: &str = "./../input/aoc_22_04/input.txt";
const INPUT_SAMPLE_PATH: &str = "./../input/aoc_22_04/input_example.txt";

// ----------------------------------------------------
// test funcs
// ----------------------------------------------------
#[test]
fn test_read_inputproper_530_id_range_pairs_containing() {
    assert_eq!(
        id_range_pairs_with_one_containing(&Path::new(INPUT_PROPER_PATH).read()),
        530,
        "aoc_22_04/input.txt has 530 id range pairs where one contains the other"
    );
}

#[test]
fn test_read_inputsample_2_id_range_pairs_containing() {
    assert_eq!(
        id_range_pairs_with_one_containing(&Path::new(INPUT_SAMPLE_PATH).read()),
        2,
        "aoc_22_04/input_example.txt has 2 id range pairs where one contains the other"
    );
}
