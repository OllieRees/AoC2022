use std::str::FromStr;

use itertools::Itertools;

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
            "X" => Ok(GameResult::Win),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(ParseMoveError)
        }
    }
}

impl GameResult {
    fn determine(opponent_move: Move, your_move: Move) -> Self {
        if opponent_move == your_move {
            return GameResult::Draw;
        }

        match (opponent_move, your_move) {
            (Move::Rock, Move::Paper) => GameResult::Win,
            (Move::Paper, Move::Scissors) => GameResult::Win,
            (Move::Scissors, Move::Rock) => GameResult::Win,
            _ => GameResult::Loss,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Round {
    opponent: Move,
    you: Move,
    result: GameResult,
}

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().map(|mv| mv.parse::<Move>()).collect_tuple::<(Result<Move, ParseMoveError>, Result<Move, ParseMoveError>)>() {
            Some(moves) => {
                match moves {
                    (Ok(opponent_move), Ok(your_move)) => Ok(Round {opponent: opponent_move, you: your_move, result: GameResult::determine(opponent_move, your_move)}),
                    _ => Err(ParseRoundError)
                }
            },
            _ => Err(ParseRoundError),
        }
    }
}

impl Round {
    fn my_score(&self) -> u32 {
        (self.result as u32) + (self.you as u32)
    }
}

fn parse_rounds(lines: Vec<String>) -> Vec<Round> {
    lines.into_iter().filter_map(|line| line.parse::<Round>().ok()).collect()
}

fn total_score(rounds: Vec<Round>) -> u32 {
    rounds.into_iter().map(|round| round.my_score()).sum::<u32>()
}

pub fn solve(lines: Vec<String>) {
    let total_score =  total_score(parse_rounds(lines));
    println!("Your total score is {total_score}");
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
    fn test_parse_round() {
        assert_eq!("A Y".parse::<Round>(), Ok(Round{opponent: Move::Rock, you: Move::Paper, result: GameResult::Win}));
        assert_eq!("B X".parse::<Round>(), Ok(Round{opponent: Move::Paper, you: Move::Rock, result: GameResult::Loss}));
        assert_eq!("C Z".parse::<Round>(), Ok(Round{opponent: Move::Scissors, you: Move::Scissors, result: GameResult::Draw}));
    }

    #[test]
    fn test_parse_rounds() {
        let lines: Vec<String> = vec![String::from("A Y"), String::from("B X"), String::from("C Z")];
        assert_eq!(parse_rounds(lines), vec![
            Round{opponent: Move::Rock, you: Move::Paper, result: GameResult::Win}, 
            Round{opponent: Move::Paper, you: Move::Rock, result: GameResult::Loss}, 
            Round{opponent: Move::Scissors, you: Move::Scissors, result: GameResult::Draw}
        ]);
    }

    #[test]
    fn test_my_score() {
        let round: Round = Round{opponent: Move::Rock, you: Move::Paper, result: GameResult::Win};
        assert_eq!(round.my_score(), 8);
        let round: Round = Round{opponent: Move::Paper, you: Move::Rock, result: GameResult::Loss};
        assert_eq!(round.my_score(), 1);
        let round: Round = Round{opponent: Move::Scissors, you: Move::Scissors, result: GameResult::Draw};
        assert_eq!(round.my_score(), 6);
    }

    #[test]
    fn test_total_score() {
        let lines: Vec<String> = vec![String::from("A Y"), String::from("B X"), String::from("C Z")];
        assert_eq!(total_score(parse_rounds(lines)), 15);
    }
}