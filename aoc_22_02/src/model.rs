pub type RPS_StrategyGuide = Vec<(RPS_Move, RPS_Move)>;

pub enum RPS_Move {
    Rock,
    Paper,
    Scissors,
}

impl RPS_Move {
    pub fn score(&self) -> u8 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}
