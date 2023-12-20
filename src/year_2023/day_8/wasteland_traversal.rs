use std::collections;

use crate::ParseInputError;

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
type Map = collections::HashMap<Node, (Node, Node)>; 

mod map {
    use std::{collections::{self, HashSet}, iter};
    use itertools::{Itertools, FoldWhile};
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::ParseInputError;
    use super::{Node, Map, Step};

    fn parse_node_and_edge(node_and_paths: String) -> Result<(Node, (Node, Node)), ParseInputError> {
        lazy_static! {
            static ref NODE_RE: Regex = Regex::new(r"(?<name>\w\w\w) = \((?<left>\w\w\w), (?<right>\w\w\w)\)").unwrap();
        }
        let caps: regex::Captures<'_> = NODE_RE.captures(&node_and_paths).ok_or(
            ParseInputError { details: format!("{node_and_paths} does not conform to the expected pattern") }
        )?;
        match (caps.name("name"), caps.name("left"), caps.name("right")) {
            (Some(name), Some(left), Some(right)) => { 
                let name: [char; 3] = name.as_str().chars().collect::<Vec<char>>().try_into().unwrap();
                let left: [char; 3] = left.as_str().chars().collect::<Vec<char>>().try_into().unwrap();
                let right: [char; 3] = right.as_str().chars().collect::<Vec<char>>().try_into().unwrap();
                Ok((name, (left, right)))
            }
            _ => Err (ParseInputError { details: "".to_string() }),
        }
    }

    pub fn parse(nodes: Vec<String>) -> Result<Map, ParseInputError> {
        type NodeMap = Vec<(Node, (Node, Node))>;
        nodes.into_iter().map(parse_node_and_edge).collect::<Result<NodeMap, ParseInputError>>().map(|x: NodeMap| collections::HashMap::from_iter(x))
    }

    pub fn execute(step: &Step, current_node: &Node, map: &Map) -> Node {
        let (l_node, r_node) = map.get(current_node).unwrap();
        match step {
            Step::Left => *l_node, 
            Step::Right => *r_node
        }
    }

    pub fn step_count(initial_node: Node, destinations: &collections::HashSet<Node>, instructions: &Vec<Step>, map: &Map) -> usize {
        let mut current_node: [char; 3] = initial_node;
        iter::repeat(instructions).flatten().fold_while(0, |acc: usize, instruction: &Step| {
            if destinations.contains(&current_node) {
                FoldWhile::Done(acc)
            } else {
                current_node = execute(instruction, &current_node, &map);
                FoldWhile::Continue(acc + 1)
            }
        }).into_inner()
    }

    pub fn step_count_multiple_starts(start_nodes: HashSet<Node>, destinations: HashSet<Node>, instructions: &Vec<Step>, map: &Map) -> usize {
        start_nodes.into_iter().map(|start| step_count(start, &destinations, &instructions, &map)).fold(1, |acc, x| num::integer::lcm(acc, x))
    }
}

fn parse_input(lines: Vec<String>) -> Result<(Vec<Step>, Map), ParseInputError> {
    let (instructions, map) = lines.split_at(2);
    let instructions: Vec<Step>= instructions::parse(instructions[0].to_string())?;
    let map: collections::HashMap<Node, (Node, Node)> = map::parse(map.to_vec())?;
    Ok((instructions, map))
}

pub fn solve(lines: Vec<String>) {
    if let Ok((instructions, map)) = parse_input(lines) {
        let step_count: usize = map::step_count(['A', 'A', 'A'], &collections::HashSet::from_iter(vec![['Z', 'Z', 'Z']]), &instructions, &map);
        println!("Number of steps taken to reach the end {step_count}");
        
        // part 2
        let start_nodes: collections::HashSet<Node> = map.keys().filter_map(|key| if *key.last().unwrap() == 'A' {Some(*key)} else {None}).collect();
        let destinations: collections::HashSet<Node> = map.keys().filter_map(|key| if *key.last().unwrap() == 'Z' {Some(*key)} else {None}).collect();
        let step_count: usize = map::step_count_multiple_starts(start_nodes, destinations, &instructions, &map);
        println!("Step Counts: {:?}", step_count);

    }
}

#[cfg(test)]
mod wasteland_tests {
    use std::collections::HashMap;

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

    #[test]
    fn succesfully_parse_map() {
        let map_lines = vec!["aaa = (bbb, ccc)".to_string(), "bbb = (ccc, zzz)".to_string()];
        let maps: Map = HashMap::from_iter([
            (['a', 'a', 'a'], (['b', 'b', 'b'], ['c', 'c', 'c'])), 
            (['b', 'b', 'b'], (['c', 'c', 'c'], ['z', 'z', 'z']))
        ]);
        assert_eq!(map::parse(map_lines), Ok(maps));
    }

    #[test]
    fn parse_map_with_one_path() {
        let map_lines = vec!["aaa = (bbb, ccc)".to_string(), "bbb = (ccc)".to_string()];
        assert!(map::parse(map_lines).is_err());
        let map_lines = vec!["aaa = (bbb, ccc)".to_string(), "bbb=ccc,ddd".to_string()];
        assert!(map::parse(map_lines).is_err());
    }

    #[test]
    fn parse_map_with_less_than_three_char_nodes() {
        let map_lines = vec!["aaa = (bbb, ccc)".to_string(), "b = (ccc, zzz)".to_string()];
        assert!(map::parse(map_lines).is_err());
    }

    #[test]
    fn parse_map_with_more_than_three_char_nodes() {
        let map_lines = vec!["aaa = (bbb, ccc)".to_string(), "bbbbbb = (ccc, zzz)".to_string()];
        let maps: Map = HashMap::from_iter([
            (['a', 'a', 'a'], (['b', 'b', 'b'], ['c', 'c', 'c'])), 
            (['b', 'b', 'b'], (['c', 'c', 'c'], ['z', 'z', 'z']))
        ]);
        assert_eq!(map::parse(map_lines), Ok(maps));
    }
}