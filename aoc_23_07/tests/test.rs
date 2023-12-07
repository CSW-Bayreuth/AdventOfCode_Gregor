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
fn test_total_winnings_of_hands_regular(
    #[case] in_path: &Path,
    #[case] expected: usize
) {
    assert_eq!(total_winnings_of_hands(read_hands(in_path), false), expected);
}

#[rstest]
#[case(Path::new("./../input/aoc_23_07/input_example.txt"), 5905)]
fn test_total_winnings_of_hands_joker(
    #[case] in_path: &Path,
    #[case] expected: usize
) {
    assert_eq!(total_winnings_of_hands(read_hands(in_path), true), expected);
}

const HAND_FIVEOFAKIND_TWOS: Hand = Hand { cards: ['2', '2', '2', '2', '2'], bid: 0 };
const HAND_FIVEOFAKIND_THREES: Hand = Hand { cards: ['3', '3', '3', '3', '3'], bid: 0 };
const HAND_FOUROFAKIND: Hand = Hand { cards: ['2', '2', '2', '2', '3'], bid: 0 };
const HAND_FULLHOUSE: Hand = Hand { cards: ['2', '2', '2', '3', '3'], bid: 0 };
const HAND_THREEOFAKIND: Hand = Hand { cards: ['2', '2', '2', '3', '4'], bid: 0 };
const HAND_TWOPAIRS: Hand = Hand { cards: ['2', '2', '3', '3', '4'], bid: 0 };
const HAND_ONEPAIR: Hand = Hand { cards: ['2', '2', '3', '4', '5'], bid: 0 };
const HAND_HIGHCARD: Hand = Hand { cards: ['1', '2', '3', '4', '5'], bid: 0 };

// ------
// regular

#[rstest]
#[case(HAND_FIVEOFAKIND_TWOS, HandType::FiveOfAKind)]
#[case(HAND_FOUROFAKIND, HandType::FourOfAKind)]
#[case(HAND_FULLHOUSE, HandType::FullHouse)]
#[case(HAND_THREEOFAKIND, HandType::ThreeOfAKind)]
#[case(HAND_TWOPAIRS, HandType::TwoPair)]
#[case(HAND_ONEPAIR, HandType::OnePair)]
#[case(HAND_HIGHCARD, HandType::HighCard)]
fn test_hand_type_regular(
    #[case] in_hand: Hand,
    #[case] expected: HandType
) {
    assert_eq!(in_hand.hand_type_regular(), expected);
}

#[rstest]
#[case(HAND_FIVEOFAKIND_TWOS, HAND_FIVEOFAKIND_TWOS, Ordering::Equal)]
#[case(HAND_FIVEOFAKIND_TWOS, HAND_FOUROFAKIND, Ordering::Greater)]
#[case(HAND_FOUROFAKIND, HAND_FIVEOFAKIND_TWOS, Ordering::Less)]
#[case(HAND_FIVEOFAKIND_TWOS, HAND_FIVEOFAKIND_THREES, Ordering::Less)]
fn test_hand_cmp_strength_regular(
    #[case] in_hand_one: Hand,
    #[case] in_hand_two: Hand,
    #[case] expected: Ordering
) {
    assert_eq!(in_hand_one.cmp_strength_regular(&in_hand_two), expected);
}

#[rstest]
#[case(vec![HAND_FIVEOFAKIND_TWOS, HAND_FOUROFAKIND, HAND_HIGHCARD, HAND_FIVEOFAKIND_THREES],
       vec![HAND_HIGHCARD, HAND_FOUROFAKIND, HAND_FIVEOFAKIND_TWOS, HAND_FIVEOFAKIND_THREES])]
fn test_hand_sorty_by_strength_regular(
    #[case] in_hands_unsorted: Vec<Hand>,
    #[case] expected: Vec<Hand>
) {
    let mut hands_to_be_sorted = in_hands_unsorted.clone();
    hands_to_be_sorted.sort_by(Hand::cmp_strength_regular);
    assert_eq!(hands_to_be_sorted, expected);
}

// ------
// joker

const HAND_JK1: Hand = Hand { cards: ['3', '2', 'T', '3', 'K'], bid: 765 };
const HAND_JK2: Hand = Hand { cards: ['T', '5', '5', 'J', '5'], bid: 684 };
const HAND_JK3: Hand = Hand { cards: ['K', 'K', '6', '7', '7'], bid: 28 };
const HAND_JK4: Hand = Hand { cards: ['K', 'T', 'J', 'J', 'T'], bid: 220 };
const HAND_JK5: Hand = Hand { cards: ['Q', 'Q', 'Q', 'J', 'A'], bid: 483 };

#[rstest]
#[case(HAND_JK1, HandType::OnePair)]
#[case(HAND_JK2, HandType::FourOfAKind)]
#[case(HAND_JK3, HandType::TwoPair)]
#[case(HAND_JK4, HandType::FourOfAKind)]
#[case(HAND_JK5, HandType::FourOfAKind)]
#[case(Hand { cards: ['Q', 'Q', 'Q', 'J', 'J'], bid: 483 }, HandType::FiveOfAKind)]
fn test_hand_type_joker(
    #[case] in_hand: Hand,
    #[case] expected: HandType
) {
    assert_eq!(in_hand.hand_type_joker(), expected);
}

#[rstest]
#[case(Hand { cards: ['J', '2', '3', '4', '5'], bid: 0 }, Hand { cards: ['2', '2', '3', '4', '5'], bid: 0 }, Ordering::Less)]
#[case(HAND_JK1, HAND_JK3, Ordering::Less)]
#[case(HAND_JK3, HAND_JK2, Ordering::Less)]
#[case(HAND_JK2, HAND_JK5, Ordering::Less)]
#[case(HAND_JK5, HAND_JK4, Ordering::Less)]
fn test_hand_cmp_strength_joker(
    #[case] in_hand_one: Hand,
    #[case] in_hand_two: Hand,
    #[case] expected: Ordering
) {
    assert_eq!(in_hand_one.cmp_strength_joker(&in_hand_two), expected);
}
#[rstest]
#[case(vec![HAND_JK1, HAND_JK2, HAND_JK3, HAND_JK4, HAND_JK5],
       vec![HAND_JK1, HAND_JK3, HAND_JK2, HAND_JK5, HAND_JK4])]
fn test_hand_sorty_by_strength_joker(
    #[case] in_hands_unsorted: Vec<Hand>,
    #[case] expected: Vec<Hand>
) {
    let mut hands_to_be_sorted = in_hands_unsorted.clone();
    hands_to_be_sorted.sort_by(Hand::cmp_strength_joker);
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
