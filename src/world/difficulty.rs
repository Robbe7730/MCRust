#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard
}

impl Into<u8> for Difficulty {
    fn into(self) -> u8 {
        match self {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3
        }
    }
}
