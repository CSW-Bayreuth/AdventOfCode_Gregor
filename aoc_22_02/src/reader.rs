use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::model::RpsMove;

// ----------------------------------------------------
pub fn read_strategy_guides(filepath: &Path) -> Vec<(RpsMove, RpsMove)> {
    read_file(filepath)
        .lines()
        .map(Result::unwrap)
        .map(read_strategy_guide)
        .collect::<Vec<(RpsMove, RpsMove)>>()
}

// ----------------------------------------------------
fn read_strategy_guide(data: String) -> (RpsMove, RpsMove) {
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
