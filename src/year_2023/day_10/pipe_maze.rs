use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use crate::ParseInputError;

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
    pub fn get_connected_positions(&self, pos: Position) -> Vec<Position> {
        let positions = || -> Vec<(i32, i32)> {
            let pos = (pos.0 as i32, pos.1 as i32);
            match self {
                Self::Start => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1), (pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
                Self::Vertical => vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
                Self::Horizontal => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
                Self::NorthEast => vec![(pos.0, pos.1 - 1), (pos.0 + 1, pos.1)],
                Self::NorthWest => vec![(pos.0, pos.1 - 1), (pos.0 - 1, pos.1)],
                Self::SouthEast => vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)],
                Self::SouthWest => vec![(pos.0, pos.1 + 1), (pos.0 - 1, pos.1)],
                Self::Ground =>vec![],
            }
        };
        positions().into_iter().filter_map(|signed_pos: (i32, i32)| {
            match (usize::try_from(signed_pos.0), usize::try_from(signed_pos.1)) {
                (Ok(x), Ok(y)) => Some((x, y)),
                _ => None 
            }
        }).collect()
    }
}

#[derive(Debug, Clone)]
struct Grid(HashMap<Position, Pipe>);

impl Grid {
    pub fn new(lines: Vec<String>) -> Result<Self, ParseInputError> {       
        let parse_row = |(row_index, line): (usize, String)| -> Result<Vec<(Position, Pipe)>, ParseInputError> { 
            line.chars().enumerate().map(|(char_index, c)| Ok(((row_index, char_index), Pipe::try_from(c)?))).collect() 
        };
        Ok(Grid(lines.into_iter().enumerate().map(parse_row).collect::<Result<Vec<Vec<(Position, Pipe)>>, _>>()?.into_iter().flatten().collect()))
    }
    
    pub fn get_cycle_from_start(&self) -> Vec<Position> {
        // https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm#Stack_invariant
        // Use a stack to find the cycle
        let start_pos = self.get_position_from_start();
        let mut dfs_stack: Vec<Position> = vec![start_pos];
        while let Some(node) = dfs_stack.pop() {
            if node == start_pos {
                return dfs_stack
            }
            self.get_connected_positions(&node).into_iter().for_each(|node: (usize, usize)| dfs_stack.push(node));
        }
        Vec::new()
    }

    pub fn get_connected_positions(&self, pos: &Position) -> Vec<Position> {
        self.0.get(pos).unwrap_or(&Pipe::Ground).get_connected_positions(*pos).into_iter().filter(|pos| self.0.contains_key(pos)).collect()
    }

    fn get_position_from_start(&self) -> Position {
        *self.0.iter().find_or_first(|(_, pipe)| **pipe==Pipe::Start).unwrap().0
    }
}


type DirectedGraph = HashMap<Position, Vec<Position>>;
impl Into<DirectedGraph> for Grid { 
    fn into(self) -> DirectedGraph {
        self.0.iter().map(|(pos, _)| (*pos, self.get_connected_positions(&pos))).collect()
    }
}


pub fn solve(lines: Vec<String>) {
    if let Ok(grid) = Grid::new(lines) {
        println!("{}", grid.get_cycle_from_start().len() / 2 as usize);
    }
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

    #[test]
    fn parse_grid_into_directed_graph() {
        let grid: Grid = Grid::new(grid()).unwrap();
        let directed_graph_grid: DirectedGraph = grid.into();
        assert_eq!(directed_graph_grid.get(&(0, 2)).unwrap(), &vec![(0, 3), (1, 2)]);
    }

    #[test]
    fn get_position_of_start() {
        let grid: Grid = Grid::new(grid()).unwrap();
        assert_eq!(grid.get_position_from_start(), (2, 0));
    }

    #[test]
    fn get_all_pipe_connected_positions_successfully() {
        assert_eq!(Pipe::Start.get_connected_positions((0, 0)), vec![(1, 0), (0, 1)]);
        assert_eq!(Pipe::Vertical.get_connected_positions((1, 1)), vec![(1, 0), (1, 2)]);
        assert_eq!(Pipe::Horizontal.get_connected_positions((1, 1)), vec![(0, 1), (2, 1)]);
        assert_eq!(Pipe::NorthEast.get_connected_positions((1, 1)), vec![(1, 0), (2, 1)]);
        assert_eq!(Pipe::NorthWest.get_connected_positions((1, 1)), vec![(1, 0), (0, 1)]);
        assert_eq!(Pipe::SouthEast.get_connected_positions((1, 1)), vec![(1, 2), (2, 1)]);
        assert_eq!(Pipe::SouthWest.get_connected_positions((1, 1)), vec![(1, 2), (0, 1)]);
        assert_eq!(Pipe::Ground.get_connected_positions((1, 1)), vec![]);
    }

    #[test]
    fn get_all_pipe_connected_positions_from_origin() {
        assert_eq!(Pipe::Start.get_connected_positions((0, 0)), vec![(1, 0), (0, 1)]);
        assert_eq!(Pipe::Vertical.get_connected_positions((0, 0)), vec![(0, 1)]);
        assert_eq!(Pipe::Horizontal.get_connected_positions((0, 0)), vec![(1, 0)]);
        assert_eq!(Pipe::NorthEast.get_connected_positions((0, 0)), vec![(1, 0)]);
        assert_eq!(Pipe::NorthWest.get_connected_positions((0, 0)), vec![]);
        assert_eq!(Pipe::SouthEast.get_connected_positions((0, 0)), vec![(0, 1), (1, 0)]);
        assert_eq!(Pipe::SouthWest.get_connected_positions((0, 0)), vec![(0, 1)]);
        assert_eq!(Pipe::Ground.get_connected_positions((0, 0)), vec![]);
    }

    #[test]
    fn get_connected_pipes_from_grid() {
        let grid: Grid = Grid::new(grid()).unwrap();
        assert_eq!(grid.get_connected_positions(&(0, 2)), vec![(0, 3), (1, 2)]);
        assert_eq!(grid.get_connected_positions(&(2, 0)), vec![(1, 0), (3, 0), (2, 1)]);
        assert_eq!(grid.get_connected_positions(&(4, 4)), vec![]);
        assert_eq!(grid.get_connected_positions(&(5, 5)), vec![]);
    }

    #[test]
    fn get_cycle_from_start_practice_grid() {
        let grid: Grid = Grid::new(grid()).unwrap();
        assert_eq!(
            grid.get_cycle_from_start(), 
            vec![
                (2, 0), (2, 1), (1, 1), (1, 2), (0, 2), (0, 3), (1, 3), (2, 3), 
                (2, 4), 
                (3, 4), (3, 3), (3, 2), (3, 1) , (4, 1), (4, 0), (3, 0), (2, 0)
            ]
        );
    }
 }