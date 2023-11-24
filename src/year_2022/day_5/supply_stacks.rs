use std::{str::FromStr, vec};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::ParseInputError;

#[derive(Debug, PartialEq, Eq)]
struct ParseGameStateError;

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;


struct GameState {
    stacks: Vec<Stack>,
}

impl FromStr for GameState {
    type Err = ParseGameStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(ParseGameStateError)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn migrate_async(&self, destination: Stack, amount: usize) -> Stack {
        Stack {crates: vec![]}
    }

    fn migrate_sync(&self, destination: Stack, amount: usize) -> Stack {
        Stack {crates: vec![]}
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    count: usize,
    source: usize,
    destination: usize,
}

impl Instruction {
    fn execute(&self, state: GameState, migrate: impl Fn(&Stack, Stack, usize) -> Stack) -> GameState {
        GameState {stacks: vec![]}
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CMD_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        if let Some(cap) = CMD_RE.captures(s) {
            let parse = |n: usize| cap.get(n).unwrap().as_str().parse::<usize>().unwrap(); // capture passes => we know there are 3 integers in the string 
            return Ok(Instruction{count: parse(1), source: parse(2), destination: parse(3)}); // logic as to why the first element is the full?!
        }
        Err(ParseInstructionError)
    }
}

fn divide_stack_instruction(input: Vec<String>) -> Result<(GameState, Vec<Option<Instruction>>), ParseInputError> {
    if let Some((state_input, instruction_input)) = input.split(|line| line == "").collect_tuple::<(&[String], &[String])>() {
        if let Ok(state) = state_input.join("\n").parse::<GameState>() {
            let instructions: Vec<Option<Instruction>> = instruction_input.into_iter().map(|instruction| instruction.parse::<Instruction>().ok()).collect();
            return Ok((state, instructions));
        }
    }
    Err(ParseInputError)
}

pub fn solve(lines: Vec<String>) {
    
}

#[cfg(test)]
mod supply_stacks {
    use crate::read_problem_input_file;
    use crate::year_2022::day_5::supply_stacks::{*};

    #[test]
    fn test_parse_instruction() {
        let input = "move 3 from 2 to 1";
        assert_eq!(input.parse::<Instruction>().unwrap(), Instruction{count: 3, source: 2, destination: 1});
    }

    #[test]
    fn test_parse_state_no_footers() {
        let mut input = read_problem_input_file("inputs/2022/5/practice.txt".to_owned());
        input.remove(input.len() - 1);
        let input = input.join("\n");
        assert!(input.parse::<GameState>().is_err()); 
    }

    #[test]
    fn test_parse_input_correct_state() {
        let input = read_problem_input_file("inputs/2022/5/practice.txt".to_owned());
        let (state, _) = divide_stack_instruction(input).unwrap();
        assert_eq!(state.stacks, vec![
            Stack{crates: vec!['N', 'Z']}, 
            Stack{crates: vec!['D', 'C', 'M']}, 
            Stack{crates: vec!['P']}
        ]);
    }

    #[test]
    fn test_parse_input_correct_instructions() {
        let input = read_problem_input_file("../../inputs/2022/5/practice.txt".to_owned());
        let (_, instructions) = divide_stack_instruction(input).unwrap();
        
        assert_eq!(instructions, vec![
            Some(Instruction{count: 1, source: 2, destination: 1}),
            Some(Instruction{count: 3, source: 1, destination: 3}),
            Some(Instruction{count: 2, source: 2, destination: 1}),
            Some(Instruction{count: 1, source: 1, destination: 2})
        ]);
    }
}