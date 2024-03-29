#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_02::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_02/input_example.txt"), HandfulOfCubes(12, 13, 14), 8)]
fn test_sum_of_possible_game_ids(
    #[case] filepath: &std::path::Path,
    #[case] cube_count: HandfulOfCubes,
    #[case] expected: usize
) {
    assert_eq!(
        sum_of_possible_game_ids(read_puzzleinput(filepath), cube_count),
        expected,
    );
}

#[rstest]
#[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
#[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", true)]
#[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", false)]
#[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", false)]
#[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
fn test_is_game_possible(
    #[case] input_line: &str,
    #[case] expected: bool
) {
    assert_eq!(
        is_game_possible(&read_game(input_line.to_string()).handfuls, HandfulOfCubes(12, 13, 14)),
        expected,
    );
}

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_02/input_example.txt"), 2286)]
fn test_power_of_min_required_cube_count(
    #[case] filepath: &std::path::Path,
    #[case] expected: usize
) {
    assert_eq!(
        power_of_min_required_cube_count(read_puzzleinput(filepath)),
        expected,
    );
}

#[rstest]
#[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", HandfulOfCubes(4, 2, 6))]
#[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", HandfulOfCubes(1, 3, 4))]
#[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", HandfulOfCubes(20, 13, 6))]
#[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", HandfulOfCubes(14, 3, 15))]
#[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", HandfulOfCubes(6, 3, 2))]
fn test_min_required_cube_count(
    #[case] input_line: &str,
    #[case] expected: HandfulOfCubes
) {
    assert_eq!(
        min_required_cube_count(&read_game(input_line.to_string()).handfuls),
        expected,
    );
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------

#[rstest]
#[case("3 blue", CubeColorCount::Blue(3))]
#[case("4 red", CubeColorCount::Red(4))]
#[case("2 green", CubeColorCount::Green(2))]
#[case("6 blue", CubeColorCount::Blue(6))]
fn test_read_cubecount(
    #[case] input_line: &str,
    #[case] expected: CubeColorCount
) {
    assert_eq!(
        read_cubecount(input_line),
        expected,
    );
}

#[rstest]
#[case("3 blue, 4 red", HandfulOfCubes(4, 0, 3))]
#[case("1 blue, 2 green", HandfulOfCubes(0, 2, 1))]
#[case("8 green, 6 blue, 20 red", HandfulOfCubes(20, 8, 6))]
#[case("1 green, 3 red, 6 blue", HandfulOfCubes(3, 1, 6))]
#[case("6 red, 1 blue, 3 green", HandfulOfCubes(6, 3, 1))]
fn test_read_handfulofcubes(
    #[case] input_line: &str,
    #[case] expected: HandfulOfCubes
) {
    assert_eq!(
        read_handfulofcubes(input_line),
        expected,
    );
}

#[rstest]
#[case(
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 
    Game{ handfuls: vec![HandfulOfCubes(4, 0, 3), HandfulOfCubes(1, 2, 6), HandfulOfCubes(0, 2, 0)], id: 1 })
]
fn test_read_game(
    #[case] input_line: &str,
    #[case] expected: Game
) {
    assert_eq!(
        read_game(input_line.to_string()),
        expected,
    );
}
