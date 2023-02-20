use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::model::{RpsMove, RpsRoundResult, RpsTwoMoves};

// ----------------------------------------------------
pub fn read_strategy_guides_nextmove(filepath: &Path) -> Vec<RpsTwoMoves> {
    read_file(filepath)
        .lines()
        .map(Result::unwrap)
        .map(read_strategy_guide_nextmove)
        .collect::<Vec<RpsTwoMoves>>()
}

// ----------------------------------------------------
pub fn read_strategy_guide_nextmove(data: String) -> RpsTwoMoves {
    assert!(data.len() == 3);
    assert!(data.chars().nth(1).unwrap() == ' ');
    (
        read_move(data.chars().nth(0).unwrap()),
        read_move(data.chars().nth(2).unwrap()),
    )
}

// ----------------------------------------------------
pub fn read_strategy_guides_nextresult(filepath: &Path) -> Vec<(RpsMove, RpsRoundResult)> {
    read_file(filepath)
        .lines()
        .map(Result::unwrap)
        .map(read_strategy_guide_nextresult)
        .collect::<Vec<(RpsMove, RpsRoundResult)>>()
}

// ----------------------------------------------------
pub fn read_strategy_guide_nextresult(data: String) -> (RpsMove, RpsRoundResult) {
    assert!(data.len() == 3);
    assert!(data.chars().nth(1).unwrap() == ' ');
    (
        read_move(data.chars().nth(0).unwrap()),
        read_result(data.chars().nth(2).unwrap()),
    )
}

// ----------------------------------------------------
fn read_move(data: char) -> RpsMove {
    match data {
        'A' | 'X' => RpsMove::Rock,
        'B' | 'Y' => RpsMove::Paper,
        'C' | 'Z' => RpsMove::Scissors,
        _ => panic!("rps strategy guide moves could not be parsed"),
    }
}

// ----------------------------------------------------
fn read_result(data: char) -> RpsRoundResult {
    match data {
        'X' => RpsRoundResult::Player1Wins,
        'Y' => RpsRoundResult::Draw,
        'Z' => RpsRoundResult::Player2Wins,
        _ => panic!("rps strategy guide results could not be parsed"),
    }
}

// ----------------------------------------------------
fn read_file(filepath: &Path) -> io::BufReader<File> {
    io::BufReader::new(File::open(filepath).unwrap())
}
