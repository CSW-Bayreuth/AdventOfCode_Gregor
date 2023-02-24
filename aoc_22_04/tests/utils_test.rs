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
// test funcs part 1
// ----------------------------------------------------
#[test]
fn test_read_inputsample_2_id_range_pairs_containing() {
    assert_eq!(
        id_range_pairs_with_one_containing(&Path::new(INPUT_SAMPLE_PATH).read()),
        2,
        "aoc_22_04/input_example.txt has 2 id range pairs where one contains the other"
    );
}

// ----------------------------------------------------
// test funcs part 2
// ----------------------------------------------------
#[test]
fn test_read_inputsample_4_id_range_pairs_containing() {
    assert_eq!(
        id_range_pairs_overlapping(&Path::new(INPUT_SAMPLE_PATH).read()),
        4,
        "aoc_22_04/input_example.txt has 4 id range pairs overlapping"
    );
}

#[test]
fn test_read_inputproper_id_range_pairs_contain_identity() {
    Path::new(INPUT_PROPER_PATH)
        .read()
        .iter()
        .for_each(|(r1, r2)| {
            assert_eq!(
                r1.overlaps(r2),
                r2.overlaps(r1),
                "overlap is an identity relation - failed with {r1}, {r2}"
            )
        })
}
