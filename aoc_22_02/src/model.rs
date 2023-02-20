#[derive(Debug)]
pub enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

impl RpsMove {
    pub fn score(&self) -> usize {
        match self {
            RpsMove::Rock => 1,
            RpsMove::Paper => 2,
            RpsMove::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum RpsRoundResult {
    Player1Wins,
    Draw,
    Player2Wins,
}

impl RpsRoundResult {
    pub fn round_score_p1(&self, player1_move: &RpsMove) -> usize {
        player1_move.score()
            + match self {
                RpsRoundResult::Player1Wins => 6,
                RpsRoundResult::Draw => 3,
                RpsRoundResult::Player2Wins => 0,
            }
    }
    pub fn round_score_p2(&self, player2_move: &RpsMove) -> usize {
        player2_move.score()
            + match self {
                RpsRoundResult::Player1Wins => 0,
                RpsRoundResult::Draw => 3,
                RpsRoundResult::Player2Wins => 6,
            }
    }
}

#[derive(Debug, Default)]
pub struct RpsTournament {
    p1_score: usize,
    p2_score: usize,
}

pub type RpsTwoMoves = (RpsMove, RpsMove);

impl RpsTournament {
    pub fn play_round(&mut self, p1_p2_moves: &RpsTwoMoves) -> &mut RpsTournament {
        let result = self.evaluate_moves(p1_p2_moves);

        self.p1_score += result.round_score_p1(&p1_p2_moves.0);
        self.p2_score += result.round_score_p2(&p1_p2_moves.1);

        self
    }

    pub fn play_rounds(&mut self, moves: Vec<RpsTwoMoves>) -> &mut RpsTournament {
        moves.iter().for_each(|p1_p2_moves| {
            self.play_round(p1_p2_moves);
        });
        self
    }

    pub fn p1_score(&self) -> usize {
        self.p1_score
    }

    pub fn p2_score(&self) -> usize {
        self.p2_score
    }

    pub fn scores(&self) -> (usize, usize) {
        (self.p1_score(), self.p2_score())
    }

    fn evaluate_moves(&self, (p1_move, p2_move): &RpsTwoMoves) -> RpsRoundResult {
        match p1_move {
            RpsMove::Rock => match p2_move {
                RpsMove::Rock => RpsRoundResult::Draw,
                RpsMove::Paper => RpsRoundResult::Player2Wins,
                RpsMove::Scissors => RpsRoundResult::Player1Wins,
            },
            RpsMove::Paper => match p2_move {
                RpsMove::Rock => RpsRoundResult::Player1Wins,
                RpsMove::Paper => RpsRoundResult::Draw,
                RpsMove::Scissors => RpsRoundResult::Player2Wins,
            },
            RpsMove::Scissors => match p2_move {
                RpsMove::Rock => RpsRoundResult::Player2Wins,
                RpsMove::Paper => RpsRoundResult::Player1Wins,
                RpsMove::Scissors => RpsRoundResult::Draw,
            },
        }
    }
}
