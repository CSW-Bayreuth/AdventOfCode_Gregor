#[derive(Debug)]
pub enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

impl RpsMove {
    pub fn score(&self) -> u8 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}
