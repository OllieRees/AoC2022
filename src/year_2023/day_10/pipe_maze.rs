use std::collections::HashMap;
use itertools::Itertools;
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
struct Grid(HashMap<Position, Pipe>);

impl Grid {
    pub fn new(lines: Vec<String>) -> Result<Self, ParseInputError> {       
        let parse_line = |(row_index, line): (usize, String)| -> Result<Vec<(Position, Pipe)>, ParseInputError> { 
            line.chars().enumerate().map(|(char_index, c)| Ok(((row_index, char_index), Pipe::try_from(c)?))).collect() 
        };
        Ok(Grid(lines.into_iter().enumerate().map(parse_line).collect::<Result<Vec<Vec<(Position, Pipe)>>, _>>()?.into_iter().flatten().collect()))
    }
    
    pub fn get_cycle_from_start(&self) -> Vec<Position> {
        Vec::new()
    }

    pub fn get_adjancet_pipes_from_position(&self, pos: &Position) -> Vec<Position> {
        Vec::new()
    }

    fn get_position_from_start(&self) -> Position {
        *self.0.iter().find_or_first(|(_, pipe)| **pipe==Pipe::Start).unwrap().0
    }
}

pub fn solve(lines: Vec<String>) {

}

#[cfg(test)]
mod pipe_maze {
    use crate::year_2023::day_10::pipe_maze::*;

    const GRID: [&str; 5] = ["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];

    fn grid() -> Vec<String> {
        GRID.into_iter().map(|s: &str| s.to_string()).collect()
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

    #[test]
    fn parse_grid_successfully() {
        let grid: Grid = Grid::new(grid()).unwrap();
        assert_eq!(grid.0.get(&(0, 0)), Some(&Pipe::Ground));
        assert_eq!(grid.0.get(&(0, 2)), Some(&Pipe::SouthEast));
        assert_eq!(grid.0.get(&(2, 0)), Some(&Pipe::Start));
        assert_eq!(grid.0.get(&(1, 3)), Some(&Pipe::Vertical));
        assert_eq!(grid.0.get(&(3, 2)), Some(&Pipe::Horizontal));
    }

    #[test]
    fn parse_grid_with_one_bad_row() {
        assert!(Grid::new(vec!["..F7.".to_string(), ".FX|.".to_string(), "SJ.L7".to_string()]).is_err());
    }
 }