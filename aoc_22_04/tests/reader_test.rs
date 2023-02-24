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
fn test_read_id_range_0_0() {
    assert_eq!(
        IdRange(0, 0),
        String::from("0-0").read(),
        "'0-0' not read as IdRange(0,0)"
    );
}

#[test]
fn test_read_id_range_0_42() {
    assert_eq!(
        IdRange(0, 42),
        String::from("0-42").read(),
        "'0-42' not read as IdRange(0,42)"
    );
}

#[test]
fn test_read_id_range_pair() {
    assert_eq!(
        (IdRange(0, 42), IdRange(7, 35)),
        String::from("0-42,7-35").read(),
        "'0-42,7-35' not read as (IdRange(0,42),IdRange(7,35))"
    );
}

#[test]
fn test_read_inputproper_1000_id_range_pairs() {
    assert_eq!(
        Path::new(INPUT_PROPER_PATH).read().len(),
        1000,
        "aoc_22_04/input.txt has 1000 entries, but a different number was parsed"
    );
}

#[test]
fn test_read_inputproper_6_id_range_pairs() {
    assert_eq!(
        Path::new(INPUT_SAMPLE_PATH).read().len(),
        6,
        "aoc_22_04/input.txt has 6 entries, but a different number was parsed"
    );
}
