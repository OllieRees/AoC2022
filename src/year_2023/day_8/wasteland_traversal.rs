use std::collections;

#[derive(Debug, PartialEq, Eq)]
enum Step {
    Left,
    Right,
}

mod instructions {
    use crate::ParseInputError;

    use super::Step;
    pub fn parse(instructions: String) -> Result<Vec<Step>, ParseInputError> {
        let parse_char_to_step = |(i, value): (usize, char)| -> Result<Step, ParseInputError> {
            match value.to_ascii_uppercase() {
                'L' => Ok(Step::Left),
                'R' => Ok(Step::Right),
                _ => Err(ParseInputError { details: format!("Cannot convert the instruction, {value}, in position {i}") }),
            }
        };
        instructions.char_indices().map(parse_char_to_step).collect()
    }
}

type Node = [char; 3];
type map = collections::HashMap<Node, (Node, Node)>; 


pub fn solve(lines: Vec<String>) {
    
}

#[cfg(test)]
mod wasteland_tests {
    use super::*;

    #[test]
    fn parse_multiple_instructions() {
        let instruction: String = "LR".to_string();
        let steps: Vec<Step> = vec![Step::Left, Step::Right];
        assert_eq!(instructions::parse(instruction), Ok(steps));
    }

    #[test]
    fn parse_single_instructions() {
        let instruction: String = "L".to_string();
        let steps: Vec<Step> = vec![Step::Left];
        assert_eq!(instructions::parse(instruction), Ok(steps));
    }

    #[test]
    fn parse_no_instructions() {
        assert_eq!(instructions::parse("".to_string()), Ok(vec![]));
    }

    #[test]
    fn parse_instructions_with_invalid_character() {
        let instruction: String = "LRXR".to_string();
        assert!(instructions::parse(instruction).is_err());
    }
}