use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::model::{RpsMove, RpsTwoMoves};

// ----------------------------------------------------
pub fn read_strategy_guides(filepath: &Path) -> Vec<RpsTwoMoves> {
    read_file(filepath)
        .lines()
        .map(Result::unwrap)
        .map(read_strategy_guide)
        .collect::<Vec<RpsTwoMoves>>()
}

// ----------------------------------------------------
fn read_strategy_guide(data: String) -> RpsTwoMoves {
    assert!(data.len() == 3);
    assert!(data.chars().nth(1).unwrap() == ' ');
    (
        read_move(data.chars().nth(0).unwrap()),
        read_move(data.chars().nth(2).unwrap()),
    )
}

// ----------------------------------------------------
fn read_move(data: char) -> RpsMove {
    match data {
        'A' | 'X' => RpsMove::Rock,
        'B' | 'Y' => RpsMove::Paper,
        'C' | 'Z' => RpsMove::Scissors,
        _ => panic!("rps strategy guide could not be parsed"),
    }
}

// ----------------------------------------------------
fn read_file(filepath: &Path) -> io::BufReader<File> {
    io::BufReader::new(File::open(filepath).unwrap())
}
