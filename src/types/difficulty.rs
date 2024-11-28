#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            &Difficulty::Peaceful => "peaceful",
            &Difficulty::Easy => "easy",
            &Difficulty::Normal => "normal",
            &Difficulty::Hard => "hard"
        }
    }
}