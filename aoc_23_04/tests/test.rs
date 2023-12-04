#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_04::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_04/input_example.txt"), 30)]
fn test_scratchcard_evolution(
    #[case] in_path: &std::path::Path,
    #[case] expected: usize
) {
    assert_eq!(scratchcard_evolution(&read_cards(in_path)), expected);
}

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_04/input_example.txt"), 13)]
fn test_worth_of_cards(
    #[case] in_path: &std::path::Path,
    #[case] expected: usize
) {
    assert_eq!(worth_of_cards(&read_cards(in_path)), expected);
}

#[rstest]
#[case(Card { id: 0, winning_numbers: vec![2], numbers_you_have: vec![3] }, 0)]
#[case(Card { id: 0, winning_numbers: vec![32, 4], numbers_you_have: vec![3434, 32] }, 1)]
#[case(Card { id: 0, winning_numbers: vec![32, 4], numbers_you_have: vec![4, 32] }, 2)]
#[case(Card { id: 0, winning_numbers: vec![32, 4, 4], numbers_you_have: vec![4, 32] }, 2)]
#[case(Card { id: 0, winning_numbers: vec![32, 4, 6], numbers_you_have: vec![4, 32, 6] }, 4)]
fn test_card_worthiness(
    #[case] card: Card,
    #[case] expected: usize
) {
    assert_eq!(card.worthiness(), expected);
}

#[rstest]
#[case(Card { id: 0, winning_numbers: vec![2], numbers_you_have: vec![3] }, vec![])]
#[case(Card { id: 0, winning_numbers: vec![32, 4], numbers_you_have: vec![3434, 32] }, vec![32])]
#[case(Card { id: 0, winning_numbers: vec![32, 4], numbers_you_have: vec![4, 32] }, vec![32, 4])]
#[case(Card { id: 0, winning_numbers: vec![32, 4, 4], numbers_you_have: vec![4, 32] }, vec![32, 4])]
#[case(Card { id: 0, winning_numbers: vec![32, 4, 6], numbers_you_have: vec![4, 32, 6] }, vec![32, 4, 6])]
fn test_card_matching_numbers(
    #[case] card: Card,
    #[case] expected: Vec<usize>
) {
    assert_eq!(card.matching_numbers(), expected);
}


// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------

#[rstest]
#[case("Card 2:  2 | 3", Card { id: 2, winning_numbers: vec![2], numbers_you_have: vec![3] })]
#[case("Card 333:   213  4 | 3434 32", Card { id: 333, winning_numbers: vec![213, 4], numbers_you_have: vec![3434, 32] })]
fn test_read_card(
    #[case] input_str: &str,
    #[case] expected: Card
) {
    assert_eq!(read_card(input_str.to_string()), expected);
}

#[rstest]
#[case("", vec![])]
#[case("3", vec![3])]
#[case(" 3", vec![3])]
#[case("3 ", vec![3])]
#[case(" 12   3   123 3", vec![12, 3, 123, 3])]
fn test_read_numbers(
    #[case] input_str: &str,
    #[case] expected: Vec<usize>
) {
    assert_eq!(read_numbers(input_str.to_string()), expected);
}
