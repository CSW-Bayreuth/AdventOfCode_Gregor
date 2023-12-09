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
    #[case] expected: u128
) {
    assert_eq!(read_plan(in_plan).steps_to_reach_end_ghost(), expected);
}

#[rstest]
#[case(Path::new("./../input/aoc_23_08/input_example2.txt"), 6)]
fn test_steps_to_reach_end_from_ghost2(
    #[case] in_plan: &Path,
    #[case] expected: usize
) {
    assert_eq!(read_plan(in_plan).steps_to_reach_end_ghost2(), expected);
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------
#[rstest]
#[case("AAA = (BBB, CCC)", Node { id: NodeId::from_str("AAA"), left: NodeId::from_str("BBB"), right: NodeId::from_str("CCC") })]
#[case("ZZZ = (ZZZ, ZZZ)", Node { id: NodeId::from_str("ZZZ"), left: NodeId::from_str("ZZZ"), right: NodeId::from_str("ZZZ") })]
#[case("HCP = (RHM, QJH)", Node { id: NodeId::from_str("HCP"), left: NodeId::from_str("RHM"), right: NodeId::from_str("QJH") })]
fn test_read_node(
    #[case] in_str: &str,
    #[case] expected: Node
) {
    assert_eq!(read_node(in_str), expected);
}

#[rstest]
#[case("RLRRRLL", vec![false, true, false, false, false, true, true])]
#[case("R", vec![false])]
#[case("L", vec![true])]
#[case("", vec![])]
fn test_read_instructions(
    #[case] in_str: &str,
    #[case] expected: Vec<Instruction>
) {
    assert_eq!(read_instructions(in_str), expected);
}

#[rstest]
#[case(vec!["AAA = (BBB, CCC)", "ZZZ = (ZZZ, ZZZ)"], HashMap::from([
    (NodeId::from_str("AAA"), Node { id: NodeId::from_str("AAA"), left: NodeId::from_str("BBB"), right: NodeId::from_str("CCC") }),
    (NodeId::from_str("ZZZ"), Node { id: NodeId::from_str("ZZZ"), left: NodeId::from_str("ZZZ"), right: NodeId::from_str("ZZZ") })
]))]
fn test_read_nodeid_to_node(
    #[case] in_lines: Vec<&str>,
    #[case] expected: HashMap<NodeId, Node>
) {
    assert_eq!(read_nodeid_to_node(in_lines), expected);
}
