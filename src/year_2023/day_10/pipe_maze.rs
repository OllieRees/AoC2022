use std::collections;

use crate::ParseInputError;

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum Pipe { Start, Vertical, Horizontal, NorthEast, NorthWest, SouthEast, SouthWest, Ground }
impl TryFrom<char> for Pipe {
    type Error = ParseInputError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            'F' => Ok(Self::SouthEast),
            '7' => Ok(Self::SouthWest),
            '.' => Ok(Self::Ground),
            _ => Err(Self::Error {details: format!("Could not parse {value} to a pipe type")}),
        }
    }
}
impl Pipe {
    pub fn connecting_positions(&self, grid_position: Position) -> Option<(Position, Position)> {
        match self {
            Self::Vertical => Some(((grid_position.0 + 1, grid_position.1), (grid_position.0 - 1, grid_position.1))),
            Self::Horizontal => Some(((grid_position.0, grid_position.1 + 1), (grid_position.0, grid_position.1 - 1))),
            Self::NorthEast => Some(((grid_position.0 + 1, grid_position.1), (grid_position.0, grid_position.1 + 1))),
            Self::NorthWest => Some(((grid_position.0 + 1, grid_position.1), (grid_position.0, grid_position.1 - 1))),
            Self::SouthEast => Some(((grid_position.0 - 1, grid_position.1), (grid_position.0, grid_position.1 + 1))),
            Self::SouthWest => Some(((grid_position.0 - 1, grid_position.1), (grid_position.0, grid_position.1 - 1))),
            Self::Ground => Some((grid_position, grid_position)),
            Self::Start => None,

        }
    }

    pub fn next_position(&self, prior_position: Position, grid_position: Position) -> Option<Position> {
        let connecting_positions = self.connecting_positions(grid_position);
        match connecting_positions {
            Some((position_1, position_2)) => if position_1 == prior_position {Some(position_2)} else {Some(position_1)},            
            None => None,
        }
    }
}

#[derive(Debug)]
struct Grid(collections::HashMap<Position, Pipe>);
impl Grid {
    pub fn new(lines: Vec<String>) -> Result<Self, ParseInputError> {       
        let parse_line = |(row_index, line): (usize, String)| -> Result<Vec<(Position, Pipe)>, ParseInputError> { 
            line.chars().enumerate().map(|(char_index, c)| {
                match Pipe::try_from(c) {
                    Ok(pipe) => Ok(((row_index, char_index), pipe)),
                    Err(e) => Err(e),
                }
            }).collect() 
        };          
        Ok(Grid(lines.into_iter().enumerate().map(parse_line).flatten().flatten().collect()))
    }

    pub fn get_start_position(&self) -> Position {
        self.0.iter().find_map(|(pos, pipe)| if *pipe == Pipe::Start {Some(*pos)} else {None}).unwrap()
    }

    pub fn get_cycle_positions(&self) -> Vec<Position> {
        // https://en.wikipedia.org/wiki/Cycle_detection#Algorithms
        Vec::new()
    }

    pub fn get_longest_distance(&self) -> usize {
        self.get_cycle_positions().len() / 2 
    }
}

pub fn solve(lines: Vec<String>) {
    if let Ok(grid) = Grid::new(lines) {
        println!("Cycle Positions: {:?}", grid.get_cycle_positions());
    }
}