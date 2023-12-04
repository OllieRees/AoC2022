use std::str::FromStr;

use itertools::Itertools;

use crate::ParseInputError;

#[derive(Debug, PartialEq, Eq)]
struct ParseMoveError;

#[derive(Debug, PartialEq, Eq)]
struct ParseRoundError;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl Move {
    fn weakness(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn strength(&self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn parse(s: &str) -> Option<Self> {
        s.parse::<Move>().ok()
    }
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(ParseMoveError)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum GameResult {
    Loss = 0,
    Win = 6,
    Draw = 3,
}

impl FromStr for GameResult {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(ParseMoveError)
        }
    }
}

impl GameResult {
    fn outcome(opponent_move: Move, your_move: Move) -> Self {
        if opponent_move == your_move {
            return GameResult::Draw;
        }
        if opponent_move.weakness() == your_move {
            return GameResult::Win;
        }
        GameResult::Loss
    }
    
    fn move_from_outcome(&self, opponent_move: Move) -> Move {
        match self {
            GameResult::Draw => opponent_move,
            GameResult::Win => opponent_move.weakness(), 
            GameResult::Loss => opponent_move.strength()
        }
    }

    fn parse(s: &str) -> Option<Self> {
        s.parse::<GameResult>().ok()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Round {
    opponent: Move,
    you: Move,
    result: GameResult,
}

impl Round {
    fn new_from_moves(line: &str) -> Option<Self> {
        match parse_line(line, Move::parse, Move::parse).ok() {
            Some((l, r)) => Some(Round{opponent: l, you: r, result: GameResult::outcome(l, r)}),
            _ => None,    
        }
    }

    fn new_from_result(line: &str) -> Option<Self> {
        match parse_line(line, Move::parse, GameResult::parse).ok() {
            Some((l, r)) => Some(Round{opponent: l, you: r.move_from_outcome(l), result: r}),
            _ => None,    
        }
    } 

    fn my_score(&self) -> u32 {
        (self.result as u32) + (self.you as u32)
    }
}

fn parse_line<L, R>(line: &str, parse_left: impl Fn(&str) -> Option<L>, parse_right: impl Fn(&str) -> Option<R>) -> Result<(L, R), ParseInputError> {
    match line.split_whitespace().collect_tuple::<(&str, &str)>() {
        Some((left_token, right_token)) => {
            match (parse_left(left_token), parse_right(right_token)) {
                (Some(x), Some(y)) => Ok((x, y)),
                _ => Err(ParseInputError {details: "Failed to parse tokens".to_string()}),
            }
        },
        _ => Err(ParseInputError {details: format!("Line was incorrectly formatted: could not split {} into two by whitespace", line)}),
    }
} 

fn parse_rounds(lines: &Vec<String>, parse_round: impl Fn(&str) -> Option<Round>) -> Vec<Round> {
    lines.into_iter().filter_map(|line| parse_round(&line[..])).collect()
}

fn total_score(rounds: Vec<Round>) -> u32 {
    rounds.into_iter().map(|round| round.my_score()).sum::<u32>()
}

pub fn solve(lines: Vec<String>) {
    println!("Total Score when moves are given: {}", total_score(parse_rounds(&lines, Round::new_from_moves)));
    println!("Total Score when moves are given: {}", total_score(parse_rounds(&lines, Round::new_from_result)));
}

#[cfg(test)]
mod rps {
    use super::*;

    #[test]
    fn test_parse_move() {
        assert_eq!("A".parse::<Move>(), Ok(Move::Rock));
        assert_eq!("B".parse::<Move>(), Ok(Move::Paper));
        assert_eq!("C".parse::<Move>(), Ok(Move::Scissors));

        assert_eq!("X".parse::<Move>(), Ok(Move::Rock));
        assert_eq!("Y".parse::<Move>(), Ok(Move::Paper));
        assert_eq!("Z".parse::<Move>(), Ok(Move::Scissors));
    }

    #[test]
    fn test_move_weakness() {
        assert_eq!(Move::Rock.weakness(), Move::Paper);
        assert_eq!(Move::Paper.weakness(), Move::Scissors);
        assert_eq!(Move::Scissors.weakness(), Move::Rock);
    }

    #[test]
    fn test_move_strength() {
        assert_eq!(Move::Rock.strength(), Move::Scissors);
        assert_eq!(Move::Paper.strength(), Move::Rock);
        assert_eq!(Move::Scissors.strength(), Move::Paper);
    }

    #[test]
    fn test_parse_game_result() {
        assert_eq!("X".parse::<GameResult>(), Ok(GameResult::Loss));
        assert_eq!("Y".parse::<GameResult>(), Ok(GameResult::Draw));
        assert_eq!("Z".parse::<GameResult>(), Ok(GameResult::Win));
    }

    #[test]
    fn test_get_result() {
        assert_eq!(GameResult::outcome(Move::Scissors, Move::Rock), GameResult::Win);
        assert_eq!(GameResult::outcome(Move::Scissors, Move::Paper), GameResult::Loss);
        assert_eq!(GameResult::outcome(Move::Scissors, Move::Scissors), GameResult::Draw);
    }

    #[test]
    fn test_get_result_from_move() {
        assert_eq!(GameResult::Win.move_from_outcome(Move::Rock), Move::Paper);
        assert_eq!(GameResult::Loss.move_from_outcome(Move::Rock), Move::Scissors);
        assert_eq!(GameResult::Draw.move_from_outcome(Move::Rock), Move::Rock);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("A Y", Move::parse, Move::parse), Ok((Move::Rock, Move::Paper)));
        assert_eq!(parse_line("A Y", Move::parse, GameResult::parse), Ok((Move::Rock, GameResult::Draw)));
    }

    #[test]
    fn parse_line_too_many_tokens() {
        let line = "A Y Z";
        assert!(parse_line(line, Move::parse, Move::parse).err().is_some());
    }

    #[test]
    fn parse_line_wrong_tokens() {
        let line = "A D";
        assert!(parse_line(line, Move::parse, Move::parse).err().is_some());
    }

    #[test]
    fn parse_line_failure_causes_round_parsing_fail() {
        // Ensure that parse line fails
        assert!(parse_line("A Y Z", Move::parse, Move::parse).err().is_some());
        
        assert!(Round::new_from_moves("A Y Z").is_none());
        assert!(Round::new_from_result("A Y Z").is_none());
    }

    #[test]
    fn test_parse_from_moves() {
        assert_eq!(Round::new_from_moves("A Y"), Some(Round{opponent: Move::Rock, you: Move::Paper, result: GameResult::Win}));
        assert_eq!(Round::new_from_moves("B X"), Some(Round{opponent: Move::Paper, you: Move::Rock, result: GameResult::Loss}));
        assert_eq!(Round::new_from_moves("C Z"), Some(Round{opponent: Move::Scissors, you: Move::Scissors, result: GameResult::Draw}));
    }

    #[test]
    fn test_parse_from_result() {
        assert_eq!(Round::new_from_result("A Y"), Some(Round{opponent: Move::Rock, you: Move::Rock, result: GameResult::Draw}));
        assert_eq!(Round::new_from_result("B X"), Some(Round{opponent: Move::Paper, you: Move::Rock, result: GameResult::Loss}));
        assert_eq!(Round::new_from_result("C Z"), Some(Round{opponent: Move::Scissors, you: Move::Rock, result: GameResult::Win}));
    }

    #[test]
    fn test_scoring() {
        let rounds: Vec<Round> = vec![
            Round{opponent: Move::Rock, you: Move::Paper, result: GameResult::Win}, 
            Round{opponent: Move::Paper, you: Move::Rock, result: GameResult::Loss},
            Round{opponent: Move::Scissors, you: Move::Scissors, result: GameResult::Draw}
        ];
        assert_eq!(rounds.get(0).unwrap().my_score(), 8);
        assert_eq!(rounds.get(1).unwrap().my_score(), 1);
        assert_eq!(rounds.get(2).unwrap().my_score(), 6);
        assert_eq!(total_score(rounds), 8 + 1 + 6);
    }
}