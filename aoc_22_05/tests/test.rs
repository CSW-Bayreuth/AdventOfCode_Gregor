// ----------------------------------------------------
// imports
// ----------------------------------------------------
// use std::path::Path;

use std::{path::Path, vec};

use aoc_22_05::*;

// ----------------------------------------------------
// tests constants
// ----------------------------------------------------
const INPUT_SAMPLE_PATH: &str = "./../input/aoc_22_05/input_example.txt";

// ----------------------------------------------------
// test funcs part 2
// ----------------------------------------------------
#[test]
fn test_top_cargos_after_cmd_apply_9001() {
    let (mut supplies, crane_cmds) = read_supply_situation(Path::new(INPUT_SAMPLE_PATH));

    assert_eq!(
        top_cargos_after_cmd_apply(&mut supplies, &crane_cmds, &CraneType::CrateMover9001),
        "MCD"
    )
}

#[test]
fn test_apply_crane_cmds_9001() {
    let mut supplies = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    apply_crane_cmds(
        &mut supplies,
        &vec![(2, 1, 2), (3, 2, 3)],
        &CraneType::CrateMover9001,
    );
    assert_eq!(
        supplies,
        vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'Z', 'N']]
    )
}

#[test]
fn test_apply_crane_cmd_9001() {
    let mut supplies = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    apply_crane_cmd(&mut supplies, &(2, 1, 2), &CraneType::CrateMover9001);
    assert_eq!(
        supplies,
        vec![vec![], vec!['M', 'C', 'D', 'Z', 'N'], vec!['P']]
    )
}

// ----------------------------------------------------
// test funcs part 1
// ----------------------------------------------------
#[test]
fn test_top_cargos_after_cmd_apply_9000() {
    let (mut supplies, crane_cmds) = read_supply_situation(Path::new(INPUT_SAMPLE_PATH));

    assert_eq!(
        top_cargos_after_cmd_apply(&mut supplies, &crane_cmds, &CraneType::CrateMover9000),
        "CMZ"
    )
}

#[test]
fn test_apply_crane_cmds_9000() {
    let mut supplies = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    apply_crane_cmds(
        &mut supplies,
        &vec![(2, 1, 2), (3, 2, 3)],
        &CraneType::CrateMover9000,
    );
    assert_eq!(
        supplies,
        vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']]
    )
}

#[test]
fn test_apply_crane_cmd_9000() {
    let mut supplies = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    apply_crane_cmd(&mut supplies, &(2, 1, 2), &CraneType::CrateMover9000);
    assert_eq!(
        supplies,
        vec![vec![], vec!['M', 'C', 'D', 'N', 'Z'], vec!['P']]
    )
}

#[test]
fn test_read_supply_situation() {
    let res: (Supplies, Vec<CraneCmd>) = read_supply_situation(Path::new(INPUT_SAMPLE_PATH));
    assert_eq!(
        res,
        (
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)]
        )
    )
}

#[test]
fn test_consume_read_supply_situation() {
    let res: (Supplies, Vec<CraneCmd>) = consume_read_supply_situation(
        &mut vec![
            String::from("    [A] [C]"),
            String::from("[G] [Z] [M] [M]"),
            String::from("[A]"),
            String::from(" 1   2   3   4 "),
            String::from(""),
            String::from("move 1 from 12 to 1"),
            String::from("move 78 from 2 to 3"),
            String::from("move 4 from 6 to 6"),
        ]
        .iter()
        .map(String::from),
    );
    assert_eq!(
        res,
        (
            vec![vec!['A', 'G'], vec!['Z', 'A'], vec!['M', 'C'], vec!['M']],
            vec![(1, 12, 1), (78, 2, 3), (4, 6, 6)]
        )
    )
}

#[test]
fn test_consume_read_supplies() {
    let res: Supplies = consume_read_supplies(
        &mut vec![
            String::from("    [A] [C]"),
            String::from("[G] [Z] [M] [M]"),
            String::from("[A]"),
            String::from(" 1   2   3   4 "),
        ]
        .iter()
        .map(String::from),
    );
    assert_eq!(
        res,
        vec![vec!['A', 'G'], vec!['Z', 'A'], vec!['M', 'C'], vec!['M']]
    )
}

#[test]
fn test_consume_read_crane_cmds() {
    let res: Vec<CraneCmd> = consume_read_crane_cmds(
        &mut vec![
            String::from("move 1 from 12 to 1"),
            String::from("move 78 from 2 to 3"),
            String::from("move 4 from 6 to 6"),
        ]
        .iter()
        .map(String::from),
    );
    assert_eq!(res, vec![(1, 12, 1), (78, 2, 3), (4, 6, 6)]);
}

#[test]
fn test_read_cargo() {
    let res: Cargo = String::from("[A]").read();
    assert_eq!(res, 'A');
}

#[test]
fn test_read_cargo_row() {
    let res: Option<CargoRow> = String::from("    [A] [C]").read();
    assert_eq!(res, Some(vec![None, Some('A'), Some('C')]));
}

#[test]
fn test_read_crane_cmd() {
    let res: CraneCmd = String::from("move 1 from 23 to 1").read();
    assert_eq!(res, (1, 23, 1));
}
