#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]

use std::path::Path;

// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_09::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(History{ values: vec![0,3,6,9,12,15] }, vec![vec![0,3,6,9,12,15], vec![3,3,3,3,3], vec![0,0,0,0]])]
fn test_history_generate_sequences(
    #[case] in_history: History,
    #[case] expected: Vec<Vec<isize>>
)
{
    assert_eq!(in_history.generate_sequences(), expected);
}

#[rstest]
#[case(History{ values: vec![0,3,6,9,12,15] }, 18)]
#[case(History{ values: vec![1,3,6,10,15,21] }, 28)]
#[case(History{ values: vec![10,13,16,21,30,45] }, 68)]
#[case(History{ values: vec![4,3,2,1,0,-1,-2,-3] }, -4)]
#[case(History{ values: vec![11,21,31,41,51,61,71,81] }, 91)]
fn test_predict_next_value(
    #[case] in_history: History,
    #[case] expected: isize
)
{
    assert_eq!(predict_next_value(in_history.generate_sequences()), expected);
}

#[rstest]
#[case(History{ values: vec![10,13,16,21,30,45] }, 5)]
fn test_predict_previous_value(
    #[case] in_history: History,
    #[case] expected: isize
)
{
    assert_eq!(predict_previous_value(in_history.generate_sequences()), expected);
}

#[rstest]
#[case(Path::new("./../input/aoc_23_09/input_example.txt"), predict_next_value, 114)]
#[case(Path::new("./../input/aoc_23_09/input_example.txt"), predict_previous_value, 2)]
fn test_sum_of_predicted_values(
    #[case] in_path: &Path,
    #[case] in_predict_func: fn(Vec<Vec<isize>>) -> isize,
    #[case] expected: isize
)
{
    assert_eq!(sum_of_predicted_values(read_histories(in_path), in_predict_func), expected);
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------

#[rstest]
#[case("0 3 6 9 12 15", History{ values: vec![0,3,6,9,12,15] })]
#[case("0 3 6 9 -12 15", History{ values: vec![0,3,6,9,-12,15] })]
fn test_read_history(
    #[case] in_str: &str,
    #[case] expected: History
)
{
    assert_eq!(read_history(in_str.to_string()), expected);
}
