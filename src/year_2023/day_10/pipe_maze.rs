use std::{collections};

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

#[derive(Debug)]
struct Grid(collections::HashMap<Position, Pipe>);

impl Grid {
    pub fn new(lines: Vec<String>) -> Result<Self, ParseInputError> {       
        Err(ParseInputError{details: "".to_string()})
    }
    
    pub fn get_cycle_from_start() -> Vec<Position> {
        Vec::new()
    }

    pub fn get_adjancet_pipes_from_position(pos: &Position) -> Vec<Position> {
        Vec::new()
    }

    fn get_position_from_start() -> Position {
        (0, 0)
    }
}

pub fn solve(lines: Vec<String>) {

}

#[cfg(test)]
mod pipe_maze {
    use crate::year_2023::day_10::pipe_maze::*;

    const GRID: [&str; 5] = ["..F7.", ".FJ|.", "SJ. L7", "|F--J", "LJ..."];

    fn parse_grid() -> Grid {
        Grid::new(GRID.into_iter().map(|s: &str| s.to_string()).collect::<Vec<String>>()).unwrap()
    }

    #[test]
    fn parse_all_pipe_types_successfully() {
        assert_eq!(Pipe::try_from('S'), Ok(Pipe::Start));
        assert_eq!(Pipe::try_from('|'), Ok(Pipe::Vertical));
        assert_eq!(Pipe::try_from('-'), Ok(Pipe::Horizontal));
        assert_eq!(Pipe::try_from('L'), Ok(Pipe::NorthEast));
        assert_eq!(Pipe::try_from('J'), Ok(Pipe::NorthWest));
        assert_eq!(Pipe::try_from('F'), Ok(Pipe::SouthEast));
        assert_eq!(Pipe::try_from('7'), Ok(Pipe::SouthWest));
        assert_eq!(Pipe::try_from('.'), Ok(Pipe::Ground));
    }

    #[test]
    fn parse_pipe_types_with_invalid_char() {
        assert_eq!(Pipe::try_from('X'), Err(ParseInputError {details: "Could not parse X to a pipe type".to_string()}));
    }
}