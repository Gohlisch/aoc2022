#[derive(Copy, Clone)]
pub enum GameOutcome {
    WIN,
    DRAW,
    LOSE
}

impl GameOutcome {
    pub fn from_char(c: char) -> Result<GameOutcome, String> {
        match c {
            'X' => Ok(GameOutcome::LOSE),
            'Y' => Ok(GameOutcome::DRAW),
            'Z' => Ok(GameOutcome::WIN),
            _ => return Err(format!("Invalid game outcome char: {}", c))
        }
    }
}
