use std::{str::FromStr, vec, collections::VecDeque};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::ParseInputError;

#[derive(Debug, PartialEq, Eq)]
struct ParseGameStateError;

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

#[derive(Debug, PartialEq, Eq)]
struct ExecutionError;


#[derive(Debug, PartialEq, Eq, Clone)]
struct Stack {
    crates: VecDeque<char>,
}

impl Stack {
    fn migrate_async(self, destination: Stack, amount: usize) -> Result<(Stack, Stack), ExecutionError> {
        let mut source_crates: VecDeque<char> = self.crates;
        let mut destination_crates: VecDeque<char> = destination.crates;
        source_crates.drain(0..amount).into_iter().for_each(|e: char| destination_crates.push_front(e));
        Ok((Stack{ crates: source_crates }, Stack{ crates: destination_crates }))
    }

    fn migrate_sync(self, destination: Stack, amount: usize) -> Result<(Stack, Stack), ExecutionError> {
        let mut source_crates: VecDeque<char> = self.crates;
        let mut destination_crates: VecDeque<char> = destination.crates;
        let mut migrating_crates: VecDeque<char> = source_crates.drain(0..amount).collect();
        migrating_crates.append(&mut destination_crates);
        Ok((Stack{ crates: source_crates }, Stack{ crates: migrating_crates }))
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    count: usize,
    source: usize,
    destination: usize,
}

impl Instruction {
    fn execute(&self, state: GameState, migrate: impl Fn(Stack, Stack, usize) -> Result<(Stack, Stack), ExecutionError>) -> Result<GameState, ExecutionError> {
        if let (Some(source_stack), Some(destination_stack)) = (state.stacks.get(self.source - 1), state.stacks.get(self.destination - 1)) {
            if let Ok((new_source_stack, new_destination_stack)) = migrate(source_stack.clone(), destination_stack.clone(), self.count) {
                // quite unsafe - can this be improved?
                let mut stacks = state.stacks.clone();
                let _ = std::mem::replace(&mut stacks[self.source - 1], new_source_stack);
                let _ = std::mem::replace(&mut stacks[self.destination - 1], new_destination_stack);
                return Ok(GameState { stacks });
            }
        }
        Err(ExecutionError)
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct GameState {
    stacks: Vec<Stack>,
}

impl GameState {
    fn _parse_stack_count(s: Option<&String>) -> Result<usize, ParseGameStateError> {
        if let Some(footer) = s {
            if let Some(x) = footer.chars().filter_map(|c: char| c.to_digit(10)).last() {
                return Ok(x as usize);
            }
        }
        Err(ParseGameStateError)
    }

    fn _parse_crates(s: Vec<String>, stack_count: usize) -> Vec<Vec<char>> {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        for line in &s[..stack_count].to_vec() {
            for (n, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                match stacks.get_mut(n) {
                    Some(stack) => stack.push(chunk[1]),
                    None => stacks.insert(n, vec![chunk[1]]),
                }
            }
        }
        stacks
    }

    fn parse(s: Vec<String>) -> Result<Self, ParseGameStateError> {
        if let Ok(stack_count) = GameState::_parse_stack_count(s.last()) {
            let create_stack = |stack: Vec<char>| Stack { crates: stack.into_iter().skip_while(|c| c == &' ').collect() };
            let stacks = GameState::_parse_crates(s, stack_count).into_iter().map(create_stack).collect();
            return Ok(GameState { stacks });
        }
        Err(ParseGameStateError)
    }

    fn top_crates(&self) -> Vec<char> {
        self.stacks.iter().map(|stack: &Stack| stack.crates.get(0).unwrap_or(&' ').to_owned()).collect()
    }
}

fn divide_stack_instruction(input: Vec<String>) -> Result<(GameState, Vec<Instruction>), ParseInputError> {
    if let Some((state_input, instruction_input)) = input.split(|line| line == "").collect_tuple::<(&[String], &[String])>() {
        if let Ok(state) = GameState::parse(state_input.to_vec()) {
            let instructions: Vec<Instruction> = instruction_input.into_iter().filter_map(|instruction| instruction.parse::<Instruction>().ok()).collect();
            return Ok((state, instructions));
        }
    }
    Err(ParseInputError {details: "Could not split the stack and instruction".to_string()})
}

pub fn solve(lines: Vec<String>) {
    if let Ok((state, instructions)) = divide_stack_instruction(lines) {
        let final_state = instructions.iter().fold(state.clone(), |state: GameState, instruction: &Instruction| instruction.execute(state, Stack::migrate_async).unwrap());
        println!("{}", String::from_iter(final_state.top_crates()));
        let final_state = instructions.iter().fold(state.clone(), |state: GameState, instruction: &Instruction| instruction.execute(state, Stack::migrate_sync).unwrap());
        println!("{}", String::from_iter(final_state.top_crates()));
    }
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
    fn test_parse_state() {
        let input = vec!["    [D]    ".to_owned(), "[N] [C]    ".to_owned(), "[Z] [M] [P]".to_owned(), " 1   2   3 ".to_owned()];
        assert_eq!(GameState::parse(input).unwrap(), GameState{stacks: vec![Stack{crates: VecDeque::from(vec!['N', 'Z'])}, Stack{crates: VecDeque::from(vec!['D', 'C', 'M'])}, Stack{crates:VecDeque::from(vec!['P'])}]});
    }

    #[test]
    fn test_parse_input_correct_state() {
        let input = read_problem_input_file("inputs/2022/5/practice.txt".to_owned());
        let (state, _) = divide_stack_instruction(input).unwrap();
        
        assert_eq!(state, GameState{stacks: vec![Stack{crates: VecDeque::from(vec!['N', 'Z'])}, Stack{crates: VecDeque::from(vec!['D', 'C', 'M'])}, Stack{crates: VecDeque::from(vec!['P'])}]});
    }

    #[test]
    fn test_parse_input_correct_instructions() {
        let input = read_problem_input_file("inputs/2022/5/practice.txt".to_owned());
        let (_, instructions) = divide_stack_instruction(input).unwrap();
        
        assert_eq!(instructions, vec![
            Instruction{count: 1, source: 2, destination: 1},
            Instruction{count: 3, source: 1, destination: 3},
            Instruction{count: 2, source: 2, destination: 1},
            Instruction{count: 1, source: 1, destination: 2}
        ]);
    }

    #[test]
    fn test_execute_migrate_async() {
        let (src, dest) = Stack{crates: VecDeque::from(vec!['D', 'N', 'Z'])}.migrate_async(Stack{crates: VecDeque::from(vec!['P'])}, 3).unwrap();
        assert_eq!(src, Stack{crates: VecDeque::new()});
        assert_eq!(dest, Stack{crates: VecDeque::from(vec!['Z', 'N', 'D', 'P'])});
    }

    #[test]
    fn test_execute_migrate_sync() {
        let (src, dest) = Stack{crates: VecDeque::from(vec!['D', 'N', 'Z'])}.migrate_sync(Stack{crates: VecDeque::from(vec!['P'])}, 3).unwrap();
        assert_eq!(src, Stack{crates: VecDeque::new()});
        assert_eq!(dest, Stack{crates: VecDeque::from(vec!['D', 'N', 'Z', 'P'])});
    }

    #[test]
    fn test_execute_instruction_async() {
        let state = GameState{stacks: vec![Stack{crates: VecDeque::from(vec!['D', 'N', 'Z'])}, Stack{crates: VecDeque::from(vec!['C', 'M'])}, Stack{crates: VecDeque::from(vec!['P'])}]};
        let new_state = Instruction{count: 3, source: 1, destination: 3}.execute(state, Stack::migrate_async).unwrap();
        assert_eq!(new_state, GameState{stacks: vec![Stack{crates: VecDeque::new()}, Stack{crates: VecDeque::from(vec!['C', 'M'])}, Stack{crates: VecDeque::from(vec!['Z', 'N', 'D', 'P'])}]});
    }

    #[test]
    fn test_execute_instruction_sync() {
        let state = GameState{stacks: vec![Stack{crates: VecDeque::from(vec!['D', 'N', 'Z'])}, Stack{crates: VecDeque::from(vec!['C', 'M'])}, Stack{crates: VecDeque::from(vec!['P'])}]};
        let new_state = Instruction{count: 3, source: 1, destination: 3}.execute(state, Stack::migrate_sync).unwrap();
        assert_eq!(new_state, GameState{stacks: vec![Stack{crates: VecDeque::new()}, Stack{crates: VecDeque::from(vec!['C', 'M'])}, Stack{crates: VecDeque::from(vec!['D', 'N', 'Z', 'P'])}]});
    }

    #[test]
    fn test_execute_solve_async() {
        let input = read_problem_input_file("inputs/2022/5/practice.txt".to_owned());
        let (state, instructions) = divide_stack_instruction(input).unwrap();
        let state = instructions.iter().fold(state.clone(), |state: GameState, instruction: &Instruction| instruction.execute(state, Stack::migrate_async).unwrap());
        assert_eq!(state, GameState{stacks: vec![Stack{crates: VecDeque::from(vec!['C'])}, Stack{crates: VecDeque::from(vec!['M'])}, Stack{crates: VecDeque::from(vec!['Z', 'N', 'D', 'P'])}]});
    }

    #[test]
    fn test_execute_solve_sync() {
        let input = read_problem_input_file("inputs/2022/5/practice.txt".to_owned());
        let (state, instructions) = divide_stack_instruction(input).unwrap();
        let state = instructions.iter().fold(state.clone(), |state: GameState, instruction: &Instruction| instruction.execute(state, Stack::migrate_sync).unwrap());
        assert_eq!(state, GameState{stacks: vec![Stack{crates: VecDeque::from(vec!['M'])}, Stack{crates: VecDeque::from(vec!['C'])}, Stack{crates: VecDeque::from(vec!['D', 'N', 'Z', 'P'])}]});
    }

}