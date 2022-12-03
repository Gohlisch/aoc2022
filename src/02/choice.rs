#[derive(Copy, Clone)]
pub enum Choice {
    ROCK,
    PAPER,
    SCISSORS
}

impl Choice {
    pub fn from_char(c: char) -> Result<Choice, String> {
        match c {
            'A' | 'X' => Ok(Choice::ROCK),
            'B' | 'Y' => Ok(Choice::PAPER),
            'C' | 'Z' => Ok(Choice::SCISSORS),
            _ => return Err(format!("Invalid action char: {}", c))
        }
    }

    pub fn from_chars(opponent: char, own: char) -> Result<(Choice, Choice), String> {
        let opponent_action = match opponent {
            'A' => Choice::ROCK,
            'B' => Choice::PAPER,
            'C' => Choice::SCISSORS,
            _ => return Err(format!("Invalid opponent char: {}", opponent))
        };
        let own_action = match own {
            'X' => Choice::ROCK,
            'Y' => Choice::PAPER,
            'Z' => Choice::SCISSORS,
            _ => return Err(format!("Invalid own char: {}", own))
        };

        Ok((opponent_action, own_action))
    }

    pub fn loses_against(self) -> Choice {
        match self {
            Choice::ROCK => Choice::PAPER,
            Choice::PAPER => Choice::SCISSORS,
            Choice::SCISSORS => Choice::ROCK
        }
    }
    
    pub fn wins_against(self) -> Choice {
        match self {
            Choice::ROCK => Choice::SCISSORS,
            Choice::PAPER => Choice::ROCK,
            Choice::SCISSORS => Choice::PAPER
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::choice::Choice;

    #[test]
    fn determines_rock() {
        let opponent = 'A';
        let own = 'X';

        let result = Choice::from_chars(opponent, own).unwrap();
        
        assert!(matches!(result.0, Choice::ROCK));
        assert!(matches!(result.1, Choice::ROCK));
    }

    #[test]
    fn determines_paper() {
        let opponent = 'B';
        let own = 'Y';

        let result = Choice::from_chars(opponent, own).unwrap();
        
        assert!(matches!(result.0, Choice::PAPER));
        assert!(matches!(result.1, Choice::PAPER));
    }

    #[test]
    fn determines_scissors() {
        let opponent = 'C';
        let own = 'Z';

        let result = Choice::from_chars(opponent, own).unwrap();
        
        assert!(matches!(result.0, Choice::SCISSORS));
        assert!(matches!(result.1, Choice::SCISSORS));
    }

    #[test]
    fn weird_opponent_input_is_err() {
        let opponent = 'F';
        let own = 'X';

        let result = Choice::from_chars(opponent, own);
        
        assert!(result.is_err());
    }

    #[test]
    fn weird_own_input_is_err() {
        let opponent = 'A';
        let own = 'F';

        let result = Choice::from_chars(opponent, own);
        
        assert!(result.is_err());
    }

    #[test]
    fn confusing_own_and_opponent_input_is_err() {
        let opponent = 'A';
        let own = 'X';

        let result = Choice::from_chars(own, opponent); // ARGUMENTS IN WRONG ORDER
        
        assert!(result.is_err());
    }
}