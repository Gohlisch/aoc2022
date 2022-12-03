mod choice;
mod game_outcome;

use std::fs;

use choice::Choice;
use game_outcome::GameOutcome;

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

    let game_codes_with_outcome = get_opponent_choice_and_outcome(&contents);

    let score_with_outcome = game_codes_with_outcome.iter().map(
        | gc| get_selection_score(&get_choice_against_opponent_choice_with_target_outcome(&gc.0, &gc.1)) + get_score_for_outcome(&gc.1)
    ).reduce(|sum, score| sum+score);

    if let Some(score_with_outcome) = score_with_outcome {
        println!("Accumulated score: {}", score_with_outcome);
    }

}

fn get_game_codes_from_input(input: &String) -> Vec<(Choice, Choice)> {
    input.lines()
        .filter(|l| l.len() >= 3)
        .map(|l| unsafe{ l.get_unchecked(0..3)} )
        .map(game_code_to_choices)
        .filter(|res| res.is_ok())
        .map(|res|res.unwrap())
        .collect()
}

fn get_opponent_choice_and_outcome(input: &String) -> Vec<(Choice, GameOutcome)> {
    input.lines()
        .filter(|l| l.len() >= 3)
        .map(|l| unsafe{ l.get_unchecked(0..3)} )
        .map(game_code_to_opponent_choice_and_game_outcome)
        .filter(|res| res.is_ok())
        .map(|res|res.unwrap())
        .collect()
}

fn get_choice_against_opponent_choice_with_target_outcome(opponent_choice: &Choice, game_outcome: &GameOutcome) -> Choice {
    match game_outcome {
        GameOutcome::WIN => opponent_choice.loses_against(),
        GameOutcome::DRAW => *opponent_choice,
        GameOutcome::LOSE => opponent_choice.wins_against()
    }
}


fn game_code_to_choices(game_code: &str) -> Result<(Choice, Choice), String> {  
    if let Some(opponent) = game_code.chars().nth(0) {
        if let Some(own) = game_code.chars().nth(2) {
            return Choice::from_chars(opponent, own);
        } 
    } 
    
    Err(format!("Invalid game code: {}", game_code))
}

fn game_code_to_opponent_choice_and_game_outcome(game_code: &str) -> Result<(Choice, GameOutcome), String> {
    if let Some(opponent) = game_code.chars().nth(0) {
        if let Some(outcome) = game_code.chars().nth(2) {
            return Ok((Choice::from_char(opponent)?, GameOutcome::from_char(outcome)?));
        } 
    } 
    
    Err(format!("Invalid game code format: \"{}\"", game_code))
}

fn get_selection_score(a: &Choice) -> i32 {
    match a {
        Choice::ROCK => 1,
        Choice::PAPER => 2,
        Choice::SCISSORS => 3
    }
}

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

fn get_round_score(opponent: &Choice, own: &Choice) -> i32 {
    match opponent {
        Choice::ROCK => match own {
            Choice::ROCK => DRAW_SCORE,
            Choice::PAPER => WIN_SCORE,
            Choice::SCISSORS => LOSE_SCORE,
        },
        Choice::PAPER => match own {
            Choice::ROCK => LOSE_SCORE,
            Choice::PAPER => DRAW_SCORE,
            Choice::SCISSORS => WIN_SCORE,
        },
        Choice::SCISSORS => match own {
            Choice::ROCK => WIN_SCORE,
            Choice::PAPER => LOSE_SCORE,
            Choice::SCISSORS => DRAW_SCORE,
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

#[cfg(test)]
mod tests {
    use crate::{Choice, get_selection_score, get_round_score, get_game_codes_from_input, get_opponent_choice_and_outcome};

    #[test]
    fn returns_game_codes_as_vector() {
        let input = String::from("A Y\nB X\nC Z");

        let game_codes = get_game_codes_from_input(&input);

        assert_eq!(game_codes.len(), 3);
        assert!(matches!(game_codes.get(0).unwrap().0, Choice::ROCK));
        assert!(matches!(game_codes.get(0).unwrap().1, Choice::PAPER));
        assert!(matches!(game_codes.get(1).unwrap().0, Choice::PAPER));
        assert!(matches!(game_codes.get(1).unwrap().1, Choice::ROCK));
        assert!(matches!(game_codes.get(2).unwrap().0, Choice::SCISSORS));
        assert!(matches!(game_codes.get(2).unwrap().1, Choice::SCISSORS));
    }

    #[test]
    fn returns_game_codes_with_target_outcome_as_vector() {
        let input = String::from("A Y\nB X\nC Z");

        let game_codes = get_opponent_choice_and_outcome(&input);

        assert_eq!(game_codes.len(), 3);
        assert!(matches!(game_codes.get(0).unwrap().0, Choice::ROCK));
        assert!(matches!(game_codes.get(0).unwrap().1, crate::GameOutcome::DRAW));
        assert!(matches!(game_codes.get(1).unwrap().0, Choice::PAPER));
        assert!(matches!(game_codes.get(1).unwrap().1, crate::GameOutcome::LOSE));
        assert!(matches!(game_codes.get(2).unwrap().0, Choice::SCISSORS));
        assert!(matches!(game_codes.get(2).unwrap().1, crate::GameOutcome::WIN));
    }

    #[test]
    fn choosing_rock_scores_1() {
        let choice = Choice::ROCK;
        
        assert_eq!(get_selection_score(&choice), 1);
    }

    #[test]
    fn choosing_paper_scores_2() {
        let choice = Choice::PAPER;
        
        assert_eq!(get_selection_score(&choice), 2);
    }

    #[test]
    fn choosing_scissors_scores_3() {
        let choice = Choice::SCISSORS;
        
        assert_eq!(get_selection_score(&choice), 3);
    }

    #[test]
    fn winning_scores_6() {
        let winning_combinations = vec![
            (Choice::ROCK, Choice::PAPER),
            (Choice::PAPER, Choice::SCISSORS),
            (Choice::SCISSORS, Choice::ROCK)  
        ];

        for opponent_own in winning_combinations {
            assert_eq!(get_round_score(&opponent_own.0, &opponent_own.1), 6);
        }
    }

    #[test]
    fn draw_scores_3() {
        let draw_combinations = vec![
            (Choice::ROCK, Choice::ROCK),
            (Choice::PAPER, Choice::PAPER),
            (Choice::SCISSORS, Choice::SCISSORS)  
        ];

        for opponent_own in draw_combinations {
            assert_eq!(get_round_score(&opponent_own.0, &opponent_own.1), 3);
        }
    }

    #[test]
    fn lose_scores_0() {
        let losing_combinations = vec![
            (Choice::ROCK, Choice::SCISSORS),
            (Choice::PAPER, Choice::ROCK),
            (Choice::SCISSORS, Choice::PAPER)  
        ];

        for opponent_own in losing_combinations {
            assert_eq!(get_round_score(&opponent_own.0, &opponent_own.1), 0);
        }
    }
}