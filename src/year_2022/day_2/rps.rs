use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn parse_move(move_symbol: &str) -> Move {
        // let move_symbol: &str = &move_symbol[..];
        match move_symbol {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Symbol given isn't linked to a move"),
        }
    }

    fn counter_move(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn beating_move(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

enum Result {
    Loss = 0,
    Win = 6,
    Draw = 3,
}

impl Result {
    fn parse_result(move_symbol: &str) -> Result {
        // let move_symbol: &str = &move_symbol[..];
        match move_symbol {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Symbol given isn't linked to a result"),
        }
    }

    fn get_result(your_move: Move, opponent_move: Move) -> Result {
        if your_move == opponent_move {
            return Result::Draw;
        }
        if opponent_move.counter_move() == your_move {
            return Result::Win;
        }
        Result::Loss
    }
}

fn get_move_from_expected_result(opponent_move: Move, expected_result: &Result) -> Move {
    match expected_result {
        Result::Draw => opponent_move,
        Result::Win => opponent_move.counter_move(),
        Result::Loss => opponent_move.beating_move(),
    }
}

fn get_score(opponent_move: Move, your_move: Move) -> u32 {
    Result::get_result(your_move, opponent_move) as u32 + your_move as u32
}

fn get_score_from_expected_result(opponent_move: Move, expected_result: Result) -> u32 {
    let your_move = get_move_from_expected_result(opponent_move, &expected_result);
    expected_result as u32 + your_move as u32
}

fn parse_move_pair(line: String) -> (Move, Move) {
    line.split_whitespace()
        .map(|move_symbol| Move::parse_move(move_symbol))
        .collect_tuple::<(Move, Move)>()
        .unwrap()
}

fn parse_move_result_pair(line: String) -> (Move, Result) {
    let (move_symbol, result_symbol): (&str, &str) =
        line.split_whitespace().collect_tuple().unwrap();

    (
        Move::parse_move(move_symbol),
        Result::parse_result(result_symbol),
    )
}

pub fn solve(lines: Vec<String>) {
    let score: u32 = lines
        .clone()
        .into_iter()
        .map(|line: String| parse_move_pair(line))
        .map(|(opp_move, your_move)| get_score(your_move, opp_move))
        .sum();

    println!("Your total score is {score}");

    let score: u32 = lines
        .into_iter()
        .map(|line: String| parse_move_result_pair(line))
        .map(|(opp_move, result)| get_score_from_expected_result(opp_move, result))
        .sum();

    println!("Your total score is {score}");
}
