use std::path::Path;

use model::RpsTournament;

mod model;
mod reader;

pub fn start_app() {
    let guide = reader::read_strategy_guides(Path::new("./input/aoc_22_02/input.txt"));

    println!(
        "Player (1,2) score: {:?}",
        RpsTournament::default().play_rounds(guide).scores()
    );
}
