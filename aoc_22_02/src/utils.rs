use crate::model::{RpsMove, RpsRoundResult, RpsTwoMoves};

pub fn eval_move_result_guide(
    (player1_move, expected_result): &(RpsMove, RpsRoundResult),
) -> RpsTwoMoves {
    (
        player1_move.clone(),
        calc_p2_move_to_get_result(&player1_move, &expected_result),
    )
}

fn calc_p2_move_to_get_result(player1_move: &RpsMove, result: &RpsRoundResult) -> RpsMove {
    match result {
        RpsRoundResult::Player1Wins => match player1_move {
            RpsMove::Paper => RpsMove::Rock,
            RpsMove::Rock => RpsMove::Scissors,
            RpsMove::Scissors => RpsMove::Paper,
        },
        RpsRoundResult::Draw => player1_move.clone(),
        RpsRoundResult::Player2Wins => match player1_move {
            RpsMove::Paper => RpsMove::Scissors,
            RpsMove::Rock => RpsMove::Paper,
            RpsMove::Scissors => RpsMove::Rock,
        },
    }
}
