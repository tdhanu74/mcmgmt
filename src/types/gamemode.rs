#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

impl GameMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            &GameMode::Survival => "survival",
            &GameMode::Creative => "creative",
            &GameMode::Adventure => "adventure",
            &GameMode::Spectator => "spectator"
        }
    }
}