use std::path::Path;

use model::RpsTournament;

use crate::model::RpsTwoMoves;

mod model;
mod reader;
mod utils;

pub fn start_app() {
    let move_move_guide =
        reader::read_strategy_guides_nextmove(Path::new("./input/aoc_22_02/input.txt"));
    let move_result_guide =
        reader::read_strategy_guides_nextresult(Path::new("./input/aoc_22_02/input.txt"));

    println!(
        "Player (1,2) score with move-move guide: {:?}",
        RpsTournament::default()
            .play_rounds(move_move_guide)
            .scores()
    );

    println!(
        "Player (1,2) score with move-result guide: {:?}",
        RpsTournament::default()
            .play_rounds(
                move_result_guide
                    .iter()
                    .map(utils::eval_move_result_guide)
                    .collect::<Vec<RpsTwoMoves>>()
            )
            .scores()
    );
}
