#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]

// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_06::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_06/input_example.txt"), 288)]
fn test_product_of_numbers_of_ways_to_beat_records(
    #[case] in_path: &std::path::Path,
    #[case] expected: usize
) {
    assert_eq!(product_of_numbers_of_ways_to_beat_records(read_race_infos(in_path)), expected);
}

#[rstest]
#[case(RaceInfo { time: 7, record_distance: 9 }, 4,)]
#[case(RaceInfo { time: 15, record_distance: 40 }, 8,)]
#[case(RaceInfo { time: 30, record_distance: 200 }, 9,)]
#[case(RaceInfo { time: 71530, record_distance: 940200 }, 71503)]
fn test_number_of_ways_to_beat_record(
    #[case] in_race_info: RaceInfo,
    #[case] expected: usize
) {
    assert_eq!(in_race_info.number_of_ways_to_beat_record(), expected);
}

#[rstest]
#[case(0, 0, 0)]
#[case(0, 1, 0)]
#[case(1, 1, 1)]
#[case(2, 1, 2)]
#[case(2, 2, 4)]
#[case(2, 3, 6)]
fn test_hold_then_travel(
    #[case] in_hold_time: usize,
    #[case] in_travel_time: usize,
    #[case] expected: usize
) {
    let mut toy = ToyBoat::default();
    toy.hold_button(in_hold_time);

    assert_eq!(toy.travel_and_return_dist(in_travel_time), expected);
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------

#[rstest]
#[case(
    std::path::Path::new("./../input/aoc_23_06/input_example.txt"), 
    RaceInfo{ time: 71530, record_distance: 940200 }
)]
fn test_read_race_info_ignore_kerning(
    #[case] in_path: &std::path::Path,
    #[case] expected: RaceInfo
) {
    assert_eq!(read_race_info_ignore_kerning(in_path), expected);
}

#[rstest]
#[case(
    std::path::Path::new("./../input/aoc_23_06/input_example.txt"), 
    vec![ 
        RaceInfo{ time: 7, record_distance: 9 },
        RaceInfo{ time: 15, record_distance: 40 },
        RaceInfo{ time: 30, record_distance: 200 },
    ])]
fn test_read_race_infos(
    #[case] in_path: &std::path::Path,
    #[case] expected: Vec<RaceInfo>
) {
    assert_eq!(read_race_infos(in_path), expected);
}
