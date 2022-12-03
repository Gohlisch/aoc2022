use std::fs;

fn main() {
    let file_path = "src/02/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read input.txt! :(");

    // Part 1

    let game_codes = get_game_codes_from_input(&contents);

    let scores = game_codes.iter().map(
        | gc| get_selection_score(&gc.1) + get_round_score(&gc.0, &gc.1)
    ).reduce(|sum, score| sum+score);

    if let Some(scores) = scores {
        println!("Accumulated score: {}", scores);
    }

    // Part 2

    let game_codes_with_outcome = get_opponent_action_and_outcome(&contents);

    let score_with_outcome = game_codes_with_outcome.iter().map(
        | gc| get_selection_score(&get_choice_against_opponent_action_with_target_outcome(&gc.0, &gc.1)) + get_score_for_outcome(&gc.1)
    ).reduce(|sum, score| sum+score);

    if let Some(score_with_outcome) = score_with_outcome {
        println!("Accumulated score: {}", score_with_outcome);
    }

}

fn get_game_codes_from_input(input: &String) -> Vec<(Action, Action)> {
    input.lines()
        .filter(|l| l.len() >= 3)
        .map(|l| unsafe{ l.get_unchecked(0..3)} )
        .map(game_code_to_actions)
        .filter(|res| res.is_ok())
        .map(|res|res.unwrap())
        .collect()
}

fn get_opponent_action_and_outcome(input: &String) -> Vec<(Action, GameOutcome)> {
    input.lines()
        .filter(|l| l.len() >= 3)
        .map(|l| unsafe{ l.get_unchecked(0..3)} )
        .map(game_code_to_opponent_action_and_game_outcome)
        .filter(|res| res.is_ok())
        .map(|res|res.unwrap())
        .collect()
}

fn get_choice_against_opponent_action_with_target_outcome(opponent_action: &Action, game_outcome: &GameOutcome) -> Action {
    match game_outcome {
        GameOutcome::WIN => get_winning_choice_against(opponent_action),
        GameOutcome::DRAW => *opponent_action,
        GameOutcome::LOSE => get_losing_choice_against(opponent_action)
    }
}

fn get_winning_choice_against(a: &Action) -> Action {
    match a {
        Action::ROCK => Action::PAPER,
        Action::PAPER => Action::SCISSORS,
        Action::SCISSORS => Action::ROCK
    }
}

fn get_losing_choice_against(a: &Action) -> Action {
    match a {
        Action::ROCK => Action::SCISSORS,
        Action::PAPER => Action::ROCK,
        Action::SCISSORS => Action::PAPER
    }
}

fn game_code_to_actions(game_code: &str) -> Result<(Action, Action), String> {  
    if let Some(opponent) = game_code.chars().nth(0) {
        if let Some(own) = game_code.chars().nth(2) {
            return Action::from_chars(opponent, own);
        } 
    } 
    
    Err(format!("Invalid game code: {}", game_code))
}

fn game_code_to_opponent_action_and_game_outcome(game_code: &str) -> Result<(Action, GameOutcome), String> {
    if let Some(opponent) = game_code.chars().nth(0) {
        if let Some(outcome) = game_code.chars().nth(2) {
            return Ok((Action::from_char(opponent)?, GameOutcome::from_char(outcome)?));
        } 
    } 
    
    Err(format!("Invalid game code format: \"{}\"", game_code))
}

fn get_selection_score(a: &Action) -> i32 {
    match a {
        Action::ROCK => 1,
        Action::PAPER => 2,
        Action::SCISSORS => 3
    }
}

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

fn get_round_score(opponent: &Action, own: &Action) -> i32 {
    match opponent {
        Action::ROCK => match own {
            Action::ROCK => DRAW_SCORE,
            Action::PAPER => WIN_SCORE,
            Action::SCISSORS => LOSE_SCORE,
        },
        Action::PAPER => match own {
            Action::ROCK => LOSE_SCORE,
            Action::PAPER => DRAW_SCORE,
            Action::SCISSORS => WIN_SCORE,
        },
        Action::SCISSORS => match own {
            Action::ROCK => WIN_SCORE,
            Action::PAPER => LOSE_SCORE,
            Action::SCISSORS => DRAW_SCORE,
        },
    }
}

fn get_score_for_outcome(outcome: &GameOutcome) -> i32 {
    match outcome {
        GameOutcome::WIN => WIN_SCORE,
        GameOutcome::DRAW => DRAW_SCORE,
        GameOutcome::LOSE => LOSE_SCORE
    }
}

#[derive(Copy, Clone)]
enum Action {
    ROCK,
    PAPER,
    SCISSORS
}

#[derive(Copy, Clone)]
enum GameOutcome {
    WIN,
    DRAW,
    LOSE
}

impl Action {
    pub fn from_char(c: char) -> Result<Action, String> {
        match c {
            'A' | 'X' => Ok(Action::ROCK),
            'B' | 'Y' => Ok(Action::PAPER),
            'C' | 'Z' => Ok(Action::SCISSORS),
            _ => return Err(format!("Invalid action char: {}", c))
        }
    }

    pub fn from_chars(opponent: char, own: char) -> Result<(Action, Action), String> {
        let opponent_action = match opponent {
            'A' => Action::ROCK,
            'B' => Action::PAPER,
            'C' => Action::SCISSORS,
            _ => return Err(format!("Invalid opponent char: {}", opponent))
        };
        let own_action = match own {
            'X' => Action::ROCK,
            'Y' => Action::PAPER,
            'Z' => Action::SCISSORS,
            _ => return Err(format!("Invalid own char: {}", own))
        };

        Ok((opponent_action, own_action))
    }
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

#[cfg(test)]
mod tests {
    use crate::{Action, get_selection_score, get_round_score, get_game_codes_from_input, get_opponent_action_and_outcome};

    #[test]
    fn returns_game_codes_as_vector() {
        let input = String::from("A Y\nB X\nC Z");

        let game_codes = get_game_codes_from_input(&input);

        assert_eq!(game_codes.len(), 3);
        assert!(matches!(game_codes.get(0).unwrap().0, Action::ROCK));
        assert!(matches!(game_codes.get(0).unwrap().1, Action::PAPER));
        assert!(matches!(game_codes.get(1).unwrap().0, Action::PAPER));
        assert!(matches!(game_codes.get(1).unwrap().1, Action::ROCK));
        assert!(matches!(game_codes.get(2).unwrap().0, Action::SCISSORS));
        assert!(matches!(game_codes.get(2).unwrap().1, Action::SCISSORS));
    }

    #[test]
    fn returns_game_codes_with_target_outcome_as_vector() {
        let input = String::from("A Y\nB X\nC Z");

        let game_codes = get_opponent_action_and_outcome(&input);

        assert_eq!(game_codes.len(), 3);
        assert!(matches!(game_codes.get(0).unwrap().0, Action::ROCK));
        assert!(matches!(game_codes.get(0).unwrap().1, crate::GameOutcome::DRAW));
        assert!(matches!(game_codes.get(1).unwrap().0, Action::PAPER));
        assert!(matches!(game_codes.get(1).unwrap().1, crate::GameOutcome::LOSE));
        assert!(matches!(game_codes.get(2).unwrap().0, Action::SCISSORS));
        assert!(matches!(game_codes.get(2).unwrap().1, crate::GameOutcome::WIN));
    }

    #[test]
    fn determines_rock() {
        let opponent = 'A';
        let own = 'X';

        let result = Action::from_chars(opponent, own).unwrap();
        
        assert!(matches!(result.0, Action::ROCK));
        assert!(matches!(result.1, Action::ROCK));
    }

    #[test]
    fn determines_paper() {
        let opponent = 'B';
        let own = 'Y';

        let result = Action::from_chars(opponent, own).unwrap();
        
        assert!(matches!(result.0, Action::PAPER));
        assert!(matches!(result.1, Action::PAPER));
    }

    #[test]
    fn determines_scissors() {
        let opponent = 'C';
        let own = 'Z';

        let result = Action::from_chars(opponent, own).unwrap();
        
        assert!(matches!(result.0, Action::SCISSORS));
        assert!(matches!(result.1, Action::SCISSORS));
    }

    #[test]
    fn weird_opponent_input_is_err() {
        let opponent = 'F';
        let own = 'X';

        let result = Action::from_chars(opponent, own);
        
        assert!(result.is_err());
    }

    #[test]
    fn weird_own_input_is_err() {
        let opponent = 'A';
        let own = 'F';

        let result = Action::from_chars(opponent, own);
        
        assert!(result.is_err());
    }

    #[test]
    fn confusing_own_and_opponent_input_is_err() {
        let opponent = 'A';
        let own = 'X';

        let result = Action::from_chars(own, opponent); // ARGUMENTS IN WRONG ORDER
        
        assert!(result.is_err());
    }

    #[test]
    fn choosing_rock_scores_1() {
        let action = Action::ROCK;
        
        assert_eq!(get_selection_score(&action), 1);
    }

    #[test]
    fn choosing_paper_scores_2() {
        let action = Action::PAPER;
        
        assert_eq!(get_selection_score(&action), 2);
    }

    #[test]
    fn choosing_scissors_scores_3() {
        let action = Action::SCISSORS;
        
        assert_eq!(get_selection_score(&action), 3);
    }

    #[test]
    fn winning_scores_6() {
        let winning_combinations = vec![
            (Action::ROCK, Action::PAPER),
            (Action::PAPER, Action::SCISSORS),
            (Action::SCISSORS, Action::ROCK)  
        ];

        for opponent_own in winning_combinations {
            assert_eq!(get_round_score(&opponent_own.0, &opponent_own.1), 6);
        }
    }

    #[test]
    fn draw_scores_3() {
        let draw_combinations = vec![
            (Action::ROCK, Action::ROCK),
            (Action::PAPER, Action::PAPER),
            (Action::SCISSORS, Action::SCISSORS)  
        ];

        for opponent_own in draw_combinations {
            assert_eq!(get_round_score(&opponent_own.0, &opponent_own.1), 3);
        }
    }

    #[test]
    fn lose_scores_0() {
        let losing_combinations = vec![
            (Action::ROCK, Action::SCISSORS),
            (Action::PAPER, Action::ROCK),
            (Action::SCISSORS, Action::PAPER)  
        ];

        for opponent_own in losing_combinations {
            assert_eq!(get_round_score(&opponent_own.0, &opponent_own.1), 0);
        }
    }
}