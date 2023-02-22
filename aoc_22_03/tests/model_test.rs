// ----------------------------------------------------
// imports
// ----------------------------------------------------
use aoc_22_03::*;

// ----------------------------------------------------
// test funcs
// ----------------------------------------------------
fn test_nodel_item_priority(test_char: char, expected_score: usize) {
    assert_eq!(
        Item(test_char).priority(),
        expected_score,
        "'{test_char}' must have score {expected_score}"
    );
}

#[test]
fn test_nodel_item_priority_a() {
    test_nodel_item_priority('a', 1);
}

#[test]
fn test_nodel_item_priority_z() {
    test_nodel_item_priority('z', 26);
}

#[test]
#[allow(non_snake_case)]
fn test_nodel_item_priority_A() {
    test_nodel_item_priority('A', 27);
}

#[test]
#[allow(non_snake_case)]
fn test_nodel_item_priority_Z() {
    test_nodel_item_priority('Z', 52);
}
