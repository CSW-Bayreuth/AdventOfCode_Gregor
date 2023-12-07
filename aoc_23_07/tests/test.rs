#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]

use std::{cmp::Ordering, path::Path};

// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_07::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(Path::new("./../input/aoc_23_07/input_example.txt"), 6440)]
fn test_total_winnings_of_hands(
    #[case] in_path: &Path,
    #[case] expected: usize
) {
    assert_eq!(total_winnings_of_hands(read_hands(in_path)), expected);
}

const HAND_FIVEOFAKIND_TWOS: Hand = Hand { cards: ['2', '2', '2', '2', '2'], bid: 0 };
const HAND_FIVEOFAKIND_THREES: Hand = Hand { cards: ['3', '3', '3', '3', '3'], bid: 0 };
const HAND_FOUROFAKIND: Hand = Hand { cards: ['2', '2', '2', '2', '3'], bid: 0 };
const HAND_FULLHOUSE: Hand = Hand { cards: ['2', '2', '2', '3', '3'], bid: 0 };
const HAND_THREEOFAKIND: Hand = Hand { cards: ['2', '2', '2', '3', '4'], bid: 0 };
const HAND_TWOPAIRS: Hand = Hand { cards: ['2', '2', '3', '3', '4'], bid: 0 };
const HAND_ONEPAIR: Hand = Hand { cards: ['2', '2', '3', '4', '5'], bid: 0 };
const HAND_HIGHCARD: Hand = Hand { cards: ['1', '2', '3', '4', '5'], bid: 0 };

#[rstest]
#[case(HAND_FIVEOFAKIND_TWOS, HandType::FiveOfAKind)]
#[case(HAND_FOUROFAKIND, HandType::FourOfAKind)]
#[case(HAND_FULLHOUSE, HandType::FullHouse)]
#[case(HAND_THREEOFAKIND, HandType::ThreeOfAKind)]
#[case(HAND_TWOPAIRS, HandType::TwoPair)]
#[case(HAND_ONEPAIR, HandType::OnePair)]
#[case(HAND_HIGHCARD, HandType::HighCard)]
fn test_hand_type(
    #[case] in_hand: Hand,
    #[case] expected: HandType
) {
    assert_eq!(in_hand.hand_type(), expected);
}

#[rstest]
#[case(HAND_FIVEOFAKIND_TWOS, HAND_FIVEOFAKIND_TWOS, Ordering::Equal)]
#[case(HAND_FIVEOFAKIND_TWOS, HAND_FOUROFAKIND, Ordering::Greater)]
#[case(HAND_FOUROFAKIND, HAND_FIVEOFAKIND_TWOS, Ordering::Less)]
#[case(HAND_FIVEOFAKIND_TWOS, HAND_FIVEOFAKIND_THREES, Ordering::Less)]
fn test_hand_cmp_strength(
    #[case] in_hand_one: Hand,
    #[case] in_hand_two: Hand,
    #[case] expected: Ordering
) {
    assert_eq!(in_hand_one.cmp_strength(&in_hand_two), expected);
}

#[rstest]
#[case(vec![HAND_FIVEOFAKIND_TWOS, HAND_FOUROFAKIND, HAND_HIGHCARD, HAND_FIVEOFAKIND_THREES],
       vec![HAND_HIGHCARD, HAND_FOUROFAKIND, HAND_FIVEOFAKIND_TWOS, HAND_FIVEOFAKIND_THREES])]
fn test_hand_sorty_by_strength(
    #[case] in_hands_unsorted: Vec<Hand>,
    #[case] expected: Vec<Hand>
) {
    let mut hands_to_be_sorted = in_hands_unsorted.clone();
    hands_to_be_sorted.sort_by(Hand::cmp_strength);
    assert_eq!(hands_to_be_sorted, expected);
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------

#[rstest]
#[case("32T3K 765", Hand { cards: ['3', '2', 'T', '3', 'K'], bid: 765 })]
fn test_read_hand(
    #[case] in_str: &str,
    #[case] expected: Hand
) {
    assert_eq!(read_hand(in_str.to_string()), expected);
}
