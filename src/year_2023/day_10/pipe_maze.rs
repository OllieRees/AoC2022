use std::{any::Any, collections::HashMap};

use crate::ParseInputError;
use itertools::Itertools;
use petgraph::{graph, graphmap, prelude, visit::IntoNodeReferences};

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Tile {
    pos: Position,
    pipe: Pipe
}

impl Tile {
    fn new(pos: Position, pipe_letter: char) -> Result<Self, ParseInputError> {
        Ok(Tile {pos, pipe: Pipe::try_from(pipe_letter)?})
    }

    fn get_connected_positions(&self) -> Vec<Position> {
        let positions = || -> Vec<(i32, i32)> {
            let pos = (self.pos.0 as i32, self.pos.1 as i32);
            match self.pipe {
                Pipe::Start => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1), (pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
                Pipe::Vertical => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
                Pipe::Horizontal => vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
                Pipe::NorthEast => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)],
                Pipe::NorthWest => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)],
                Pipe::SouthEast => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
                Pipe::SouthWest => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)],
                Pipe::Ground =>vec![],
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
struct PipeMaze(graphmap::DiGraphMap<Tile, ()>);

impl TryFrom<Vec<String>> for PipeMaze {
    type Error = ParseInputError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let nodes: Vec<Tile> = value.into_iter().enumerate().map(|(row_n, row): (usize, String)|
            row.chars().enumerate().map(|(column_n, pipe_letter): (usize, char)| 
                Tile::new((row_n, column_n), pipe_letter)
            ).collect()
        ).collect::<Result<Vec<Vec<Tile>>, ParseInputError>>()?.into_iter().flatten().collect();
        Ok(Self::from(nodes))
    }
}

impl From<Vec<Tile>> for PipeMaze {
    fn from(nodes: Vec<Tile>) -> Self {
        let mut grid: graphmap::DiGraphMap<Tile, ()> = graphmap::DiGraphMap::new();
        let position_map: HashMap<Position, Tile> = nodes.iter().map(|node: &Tile| (node.pos, *node)).collect();

        let get_neighbours_for_node = |node: &Tile| -> Vec<&Tile> {
            node.get_connected_positions().iter().filter_map(|pos: &(usize, usize)| position_map.get(pos)).collect()
        };

        for node in nodes.iter() {
            if node.pipe != Pipe::Ground {
                grid.add_node(*node);
                for neighbour in get_neighbours_for_node(node) {
                    grid.add_edge(*node, *neighbour, ());
                }
            }
        }
        PipeMaze(grid)
    }
}

impl PipeMaze {
    pub fn get_tile(&self, position: Position) -> Option<Tile> {
        self.0.nodes().find_or_first(|tile| tile.pos == position)
    }

    pub fn get_neighbours(&self, tile: &Tile) -> Vec<Tile> {
        self.0.neighbors(self.0.node_references().find_or_first(|(node, _)| node == tile).unwrap().0).collect()
    }

    pub fn get_cycle_from_start(&self) -> Option<Vec<Tile>> {
        let graph: prelude::Graph<Tile, ()> = self.0.to_owned().into_graph::<u32>();
        let start_node_index: prelude::NodeIndex = graph.node_indices().find(|i: &prelude::NodeIndex| graph[*i].pipe == Pipe::Start).unwrap();
        
        let cycles_with_start_node: Vec<Vec<Tile>> = petgraph::algo::tarjan_scc(&graph).into_iter().filter(
            |cycle: &Vec<graph::NodeIndex<_>>| cycle.contains(&start_node_index) // Only cycles with start
        ).map(
            |cycle: Vec<graph::NodeIndex>| cycle.into_iter().map(|i| graph[i]).collect()  // node index -> tile
        ).collect();

        cycles_with_start_node.into_iter().max_by(
            |x: &Vec<Tile>, y: &Vec<Tile>| x.len().cmp(&y.len())  // Get longest cycle
        )
    }
}


pub fn solve(lines: Vec<String>) {
    if let Ok(grid) = PipeMaze::try_from(lines) {
        if let Some(cycle)  = grid.get_cycle_from_start() {
            println!("{}", cycle.len() / 2 as usize);
        }
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
    fn new_node() {
        assert_eq!(Tile::new((0, 0), 'S').unwrap(), Tile {pos: (0, 0), pipe: Pipe::Start});
    }

    #[test]
    fn new_node_err() {
        assert!(Tile::new((0, 0), 'X').is_err());
    }


    #[test]
    fn parse_grid_nodes_successfully() {
        let grid_3x3: PipeMaze = PipeMaze::try_from(vec![String::from("S-7"), String::from("|.|"), String::from("L-J")]).unwrap();
        assert!(grid_3x3.0.contains_node(Tile {pos: (0, 0), pipe: Pipe::Start}));
        assert!(grid_3x3.0.contains_node(Tile {pos: (0, 1), pipe: Pipe::Horizontal}));
        assert!(grid_3x3.0.contains_node(Tile {pos: (0, 2), pipe: Pipe::SouthWest}));
        assert!(grid_3x3.0.contains_node(Tile {pos: (1, 0), pipe: Pipe::Vertical}));
        assert!(grid_3x3.0.contains_node(Tile {pos: (1, 2), pipe: Pipe::Vertical}));
        assert!(grid_3x3.0.contains_node(Tile {pos: (2, 0), pipe: Pipe::NorthEast}));
        assert!(grid_3x3.0.contains_node(Tile {pos: (2, 1), pipe: Pipe::Horizontal}));
        assert!(grid_3x3.0.contains_node(Tile {pos: (2, 2), pipe: Pipe::NorthWest}));
    }

    #[test]
    fn parse_grid_neighbours_successfully() {
        let grid: PipeMaze = PipeMaze::try_from(vec![String::from("S-7"), String::from("|.|"), String::from("L-J")]).unwrap();
        assert_eq!(
            grid.get_neighbours(&Tile { pos: (0, 0), pipe: Pipe::Start }), 
            vec![Tile {pos: (1, 0), pipe: Pipe::Vertical}, Tile {pos: (0, 1), pipe: Pipe::Horizontal}]
        );

        let grid: PipeMaze = PipeMaze::try_from(vec![String::from("..."), String::from(".S."), String::from("...")]).unwrap();
        assert_eq!(
            grid.get_neighbours(&Tile { pos: (1, 1), pipe: Pipe::Start }), 
            vec![
                Tile { pos: (0, 1), pipe: Pipe::Ground }, 
                Tile { pos: (2, 1), pipe: Pipe::Ground }, 
                Tile { pos: (1, 0), pipe: Pipe::Ground }, 
                Tile { pos: (1, 2), pipe: Pipe::Ground }
            ]
        );
        let grid: PipeMaze = PipeMaze::try_from(vec![String::from("LL."), String::from("..."), String::from("..S")]).unwrap();
        assert_eq!(
            grid.get_neighbours(&Tile { pos: (2, 2), pipe: Pipe::Start }), 
            vec![
                Tile { pos: (1, 2), pipe: Pipe::Ground }, 
                Tile { pos: (2, 1), pipe: Pipe::Ground }, 
            ]
        );
        assert_eq!(
            grid.get_neighbours(&Tile { pos: (0, 1), pipe: Pipe::NorthEast }), 
            vec![Tile { pos: (0, 2), pipe: Pipe::Ground }]
        );
        assert_eq!(
            grid.get_neighbours(&Tile { pos: (0, 0), pipe: Pipe::NorthEast }), 
            vec![Tile { pos: (0, 1), pipe: Pipe::NorthEast }]
        );
    }

    #[test]
    fn parse_grid_with_one_bad_row() {
        assert!(PipeMaze::try_from(vec!["..F7.".to_string(), ".FX|.".to_string(), "SJ.L7".to_string()]).is_err());
    }

    #[test]
    fn get_indicies() {
        let grid: PipeMaze = PipeMaze::try_from(grid()).unwrap();
        println!("{:?}", grid.0.into_graph::<usize>().node_indices().nth(0));
    }


    #[test]
    fn get_cycle_from_start_practice_grid() {
        let grid: PipeMaze = PipeMaze::try_from(grid()).unwrap();
        assert_eq!(grid.get_cycle_from_start().unwrap().len(), 16);
    }
 }
