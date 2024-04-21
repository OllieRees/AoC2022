use crate::ParseInputError;
use petgraph::graphmap::DiGraphMap;


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


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Node {
    pos: Position,
    pipe: Pipe
}

impl Node {
    fn new(pos: Position, pipe_letter: char) -> Result<Self, ParseInputError> {
        Ok(Node {pos, pipe: Pipe::try_from(pipe_letter)?})
    }
}


#[derive(Debug, Clone)]
struct PipeMaze(DiGraphMap<Node, ()>);

impl TryFrom<Vec<String>> for PipeMaze {
    type Error = ParseInputError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut grid: DiGraphMap<Node, ()> = DiGraphMap::new();

        let nodes: Vec<Node> = value.into_iter().enumerate().map(|(row_n, row): (usize, String)| 
            row.chars().enumerate().map(|(column_n, pipe_letter): (usize, char)| 
                Node::new((row_n, column_n), pipe_letter)
            ).collect()
        ).collect::<Result<Vec<Vec<Node>>, ParseInputError>>()?.into_iter().flatten().collect();
        
        let get_neighbours = |node: &Node| -> Vec<&Node> {
            nodes.iter().filter(|neighbour| node.pipe.get_connected_positions(node.pos).contains(&neighbour.pos)).collect()
        };
        
        // populate graph
        for node in nodes.iter() {
            grid.add_node(*node);
            for neighbour in  get_neighbours(node){
                grid.add_edge(*node, *neighbour, ());
            }
        }
        Ok(PipeMaze(grid))
    }   
}

impl PipeMaze {
    pub fn get_cycle_from_start(&self) -> Vec<Node> {
        // https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm#Stack_invariant
        // Use a stack to find the cycle
        Vec::new()
    }

    pub fn get_connected_positions(&self, pos: &Position) -> Vec<Position> {
        Vec::new()
        // self.0.get(pos).unwrap_or(&Pipe::Ground).get_connected_positions(*pos).into_iter().filter(|pos| self.0.contains_key(pos)).collect()
    }

    fn get_position_from_start(&self) -> Position {
        (0, 0)
        // *self.0.iter().find_or_first(|(node, _)| node.pipe==Pipe::Start).unwrap().0.pos
    }
}


pub fn solve(lines: Vec<String>) {
    if let Ok(grid) = PipeMaze::try_from(lines) {
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
    fn get_all_pipe_connected_positions_successfully() {
        assert_eq!(Pipe::Start.get_connected_positions((1, 1)), vec![(0, 1), (2, 1), (1, 0), (1, 2)]);
        assert_eq!(Pipe::Vertical.get_connected_positions((1, 1)), vec![(1, 0), (1, 2)]);
        assert_eq!(Pipe::Horizontal.get_connected_positions((1, 1)), vec![(0, 1), (2, 1)]);
        assert_eq!(Pipe::NorthEast.get_connected_positions((1, 1)), vec![(1, 0), (2, 1)]);
        assert_eq!(Pipe::NorthWest.get_connected_positions((1, 1)), vec![(1, 0), (0, 1)]);
        assert_eq!(Pipe::SouthEast.get_connected_positions((1, 1)), vec![(1, 2), (2, 1)]);
        assert_eq!(Pipe::SouthWest.get_connected_positions((1, 1)), vec![(1, 2), (0, 1)]);
        assert_eq!(Pipe::Ground.get_connected_positions((1, 1)), vec![]);
    }

    #[test]
    fn get_all_pipe_connected_positions_from_corner() {
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
    fn new_node() {
        assert_eq!(Node::new((0, 0), 'S').unwrap(), Node {pos: (0, 0), pipe: Pipe::Start});
    }

    #[test]
    fn new_node_err() {
        assert!(Node::new((0, 0), 'X').is_err());
    }


    #[test]
    fn parse_grid_nodes_successfully() {
        let grid_3x3: PipeMaze = PipeMaze::try_from(vec![String::from("S-7"), String::from("|.|"), String::from("L-J")]).unwrap();
        assert!(grid_3x3.0.contains_node(Node {pos: (0, 0), pipe: Pipe::Start}));
        assert!(grid_3x3.0.contains_node(Node {pos: (0, 1), pipe: Pipe::Horizontal}));
        assert!(grid_3x3.0.contains_node(Node {pos: (0, 2), pipe: Pipe::SouthWest}));
        assert!(grid_3x3.0.contains_node(Node {pos: (1, 0), pipe: Pipe::Vertical}));
        assert!(grid_3x3.0.contains_node(Node {pos: (1, 1), pipe: Pipe::Ground}));
        assert!(grid_3x3.0.contains_node(Node {pos: (1, 2), pipe: Pipe::Vertical}));
        assert!(grid_3x3.0.contains_node(Node {pos: (2, 0), pipe: Pipe::NorthEast}));
        assert!(grid_3x3.0.contains_node(Node {pos: (2, 1), pipe: Pipe::Horizontal}));
        assert!(grid_3x3.0.contains_node(Node {pos: (2, 2), pipe: Pipe::NorthWest}));
    }

    #[test]
    fn parse_grid_edges_successfully() {
        let grid: PipeMaze = PipeMaze::try_from(vec![String::from("S-7"), String::from("|.|"), String::from("L-J")]).unwrap();
        assert!(grid.0.edge_count() == 8);  
        let grid: PipeMaze = PipeMaze::try_from(vec![String::from("S.7"), String::from("L.|"), String::from("..L")]).unwrap();
        assert!(grid.0.edge_count() == 5);  
    }

    #[test]
    fn parse_grid_with_one_bad_row() {
        assert!(PipeMaze::try_from(vec!["..F7.".to_string(), ".FX|.".to_string(), "SJ.L7".to_string()]).is_err());
    }

    // #[test]
    // fn get_position_of_start() {
    //     let grid: Grid = Grid::try_from(grid()).unwrap();
    //     assert_eq!(grid.get_position_from_start(), (2, 0));
    // }

    // #[test]
    // fn get_connected_pipes_from_grid() {
    //     let grid: Grid = Grid::from_lines(grid()).unwrap();
    //     assert_eq!(grid.get_connected_positions(&(0, 2)), vec![(0, 3), (1, 2)]);
    //     assert_eq!(grid.get_connected_positions(&(2, 0)), vec![(1, 0), (3, 0), (2, 1)]);
    //     assert_eq!(grid.get_connected_positions(&(4, 4)), vec![]);
    //     assert_eq!(grid.get_connected_positions(&(5, 5)), vec![]);
    // }

    // #[test]
    // fn get_cycle_from_start_practice_grid() {
    //     let grid: Grid = Grid::from_lines(grid()).unwrap();
    //     assert_eq!(
    //         grid.get_cycle_from_start(), 
    //         vec![
    //             (2, 0), (2, 1), (1, 1), (1, 2), (0, 2), (0, 3), (1, 3), (2, 3), 
    //             (2, 4), 
    //             (3, 4), (3, 3), (3, 2), (3, 1) , (4, 1), (4, 0), (3, 0), (2, 0)
    //         ]
    //     );
    // }
 }