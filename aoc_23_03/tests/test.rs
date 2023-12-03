#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_03::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_03/input_example.txt"), 467835)]
#[case(std::path::Path::new("./../input/aoc_23_03/input_example2.txt"), 0)]
fn test_sum_of_gear_ratios(
    #[case] filepath: &std::path::Path,
    #[case] expected: usize
) {
    assert_eq!(sum_of_gear_ratios(filepath), expected);
}

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_03/input_example.txt"), 4361)]
#[case(std::path::Path::new("./../input/aoc_23_03/input_example2.txt"), 18)]
fn test_sum_of_part_numbers(
    #[case] filepath: &std::path::Path,
    #[case] expected: usize
) {
    assert_eq!(sum_of_part_numbers(filepath), expected);
}

#[rstest]
#[case(
    SchemaGrid { size_x: 5, size_y: 1, 
        cells: vec![
            vec![ SchemaCell::Empty, SchemaCell::Number(7), SchemaCell::Number(3), SchemaCell::Symbol, SchemaCell::Number(1) ],
            vec![ SchemaCell::Number(7), SchemaCell::Empty, SchemaCell::Symbol, SchemaCell::Symbol, SchemaCell::Symbol ]
        ] 
    }, 
    vec![SchemaNumber{ number: 73, x_start: 1, x_end: 2, y: 0 }, SchemaNumber{ number: 1, x_start: 4, x_end: 4, y: 0 }, SchemaNumber{ number: 7, x_start: 0, x_end: 0, y: 1 }]
)]
fn test_extract_schema_numbers(
    #[case] input_schema_grid: SchemaGrid,
    #[case] expected: Vec<SchemaNumber>
) {
    let mut schema_grid = input_schema_grid;

    assert_eq!(
        extract_schema_numbers(&mut schema_grid),
        expected
     )
}

#[rstest]
#[case( // #1
    SchemaNumber{ number: 7, x_start: 0, x_end: 0, y: 0 },
    SchemaGrid{ size_x: 1, size_y: 1, cells: vec![vec![SchemaCell::Number(7)]] },
    false
)]
#[case( // #2
    SchemaNumber{ number: 7, x_start: 0, x_end: 0, y: 0 },
    SchemaGrid{ size_x: 2, size_y: 1, cells: vec![vec![SchemaCell::Number(7), SchemaCell::Empty]] },
    false
)]
#[case( // #3 right to num
    SchemaNumber{ number: 7, x_start: 0, x_end: 0, y: 0 },
    SchemaGrid{ size_x: 2, size_y: 1, cells: vec![vec![SchemaCell::Number(7), SchemaCell::Symbol]] },
    true
)]
#[case( // #4 left to num
    SchemaNumber{ number: 7, x_start: 1, x_end: 1, y: 0 },
    SchemaGrid{ size_x: 2, size_y: 1, cells: vec![vec![SchemaCell::Symbol, SchemaCell::Number(7)]] },
    true
)]
#[case( // #5 below num
    SchemaNumber{ number: 7, x_start: 0, x_end: 0, y: 0 },
    SchemaGrid{ size_x: 1, size_y: 2, cells: vec![vec![SchemaCell::Number(7)], vec![SchemaCell::Symbol]] },
    true
)]
#[case( // #6 above num
    SchemaNumber{ number: 7, x_start: 0, x_end: 0, y: 1 },
    SchemaGrid{ size_x: 1, size_y: 2, cells: vec![vec![SchemaCell::Symbol], vec![SchemaCell::Number(7)]] },
    true
)]
#[case( // #7 symbol top-left to num
    SchemaNumber{ number: 7, x_start: 0, x_end: 0, y: 1 },
    SchemaGrid{ size_x: 2, size_y: 2, cells: vec![
        vec![SchemaCell::Symbol, SchemaCell::Empty], 
        vec![SchemaCell::Empty, SchemaCell::Number(7)]] },
    true
)]
#[case( // #8 num top-left to num
    SchemaNumber{ number: 7, x_start: 1, x_end: 1, y: 1 },
    SchemaGrid{ size_x: 2, size_y: 2, cells: vec![
        vec![SchemaCell::Number(1), SchemaCell::Empty], 
        vec![SchemaCell::Empty, SchemaCell::Number(7)]] },
        false
)]
#[case( // #9 num top-right to num
    SchemaNumber{ number: 7, x_start: 0, x_end: 0, y: 1 },
    SchemaGrid{ size_x: 2, size_y: 2, cells: vec![
        vec![SchemaCell::Empty, SchemaCell::Symbol], 
        vec![SchemaCell::Number(7), SchemaCell::Empty]] },
    true
)]
#[case( // #10 symbol top-right to long num
    SchemaNumber{ number: 723, x_start: 0, x_end: 2, y: 1 },
    SchemaGrid{ size_x: 4, size_y: 2, cells: vec![
        vec![SchemaCell::Empty, SchemaCell::Empty, SchemaCell::Empty, SchemaCell::Symbol], 
        vec![SchemaCell::Number(7), SchemaCell::Number(2), SchemaCell::Number(3), SchemaCell::Empty] 
    ]},
    true
)]
fn test_is_next_to_symbol(
    #[case] input_schema_num: SchemaNumber,
    #[case] input_schema_grid: SchemaGrid,
    #[case] expected: bool
) {
    let mut schema_grid = input_schema_grid;

    assert_eq!(
        input_schema_num.is_next_to_symbol(&mut schema_grid),
        expected
     )
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------

#[rstest]
#[case(
    std::path::Path::new("./../input/aoc_23_03/input_example2.txt"), 
    Box::new(SchemaGrid { 
        size_x: 5, 
        size_y: 2, 
        cells: vec![
            vec![SchemaCell::Number(7), SchemaCell::Empty, SchemaCell::Empty, SchemaCell::Number(1), SchemaCell::Number(1)], 
            vec![SchemaCell::Empty, SchemaCell::Symbol, SchemaCell::Empty, SchemaCell::SymbolPotentialGear(vec![]), SchemaCell::Empty]
        ] 
    })
)]
fn test_read_schemagrid(
    #[case] filepath: &std::path::Path,
    #[case] expected: Box<SchemaGrid>
) {
    assert_eq!(
        read_schemagrid(filepath),
        expected,
    );
}

#[rstest]
#[case('.', SchemaCell::Empty)]
#[case('0', SchemaCell::Number(0))]
#[case('9', SchemaCell::Number(9))]
#[case('!', SchemaCell::Symbol)]
#[case('*', SchemaCell::SymbolPotentialGear(vec![]))]
fn test_read_schemacell(
    #[case] input_char: char,
    #[case] expected: SchemaCell
) {
    assert_eq!(
        read_schemacell(input_char),
        expected
     )
}
