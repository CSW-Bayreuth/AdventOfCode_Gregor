#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]

use std::{path::Path, collections::HashMap};

// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_08::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(Path::new("./../input/aoc_23_08/input_example.txt"), 2)]
fn test_steps_to_reach_end_from_aaa(
    #[case] in_plan: &Path,
    #[case] expected: usize
) {
    assert_eq!(read_plan(in_plan).steps_to_reach_end_from_aaa(), expected);
}

#[rstest]
#[case(Path::new("./../input/aoc_23_08/input_example2.txt"), 6)]
fn test_steps_to_reach_end_from_ghost(
    #[case] in_plan: &Path,
    #[case] expected: usize
) {
    assert_eq!(read_plan(in_plan).steps_to_reach_end_ghost(), expected);
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------
#[rstest]
#[case("AAA = (BBB, CCC)", Node { id: "AAA".to_string(), left: "BBB".to_string(), right: "CCC".to_string() })]
#[case("ZZZ = (ZZZ, ZZZ)", Node { id: "ZZZ".to_string(), left: "ZZZ".to_string(), right: "ZZZ".to_string() })]
#[case("HCP = (RHM, QJH)", Node { id: "HCP".to_string(), left: "RHM".to_string(), right: "QJH".to_string() })]
fn test_read_node(
    #[case] in_str: &str,
    #[case] expected: Node
) {
    assert_eq!(read_node(in_str), expected);
}

#[rstest]
#[case("RLRRRLL", vec!['R', 'L', 'R', 'R', 'R', 'L', 'L'])]
#[case("R", vec!['R'])]
#[case("L", vec!['L'])]
#[case("", vec![])]
fn test_read_instructions(
    #[case] in_str: &str,
    #[case] expected: Vec<Instruction>
) {
    assert_eq!(read_instructions(in_str), expected);
}

#[rstest]
#[case(vec!["AAA = (BBB, CCC)", "ZZZ = (ZZZ, ZZZ)"], HashMap::from([
    ("AAA".to_string(), Node { id: "AAA".to_string(), left: "BBB".to_string(), right: "CCC".to_string() }),
    ("ZZZ".to_string(), Node { id: "ZZZ".to_string(), left: "ZZZ".to_string(), right: "ZZZ".to_string() })
]))]
fn test_read_nodeid_to_node(
    #[case] in_lines: Vec<&str>,
    #[case] expected: HashMap<String, Node>
) {
    assert_eq!(read_nodeid_to_node(in_lines), expected);
}
