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
    pub fn connecting_positions(&self, grid_position: &Position) -> Option<(Position, Position)> {
        match self {
            Self::Vertical => Some(((grid_position.0 + 1, grid_position.1), (grid_position.0 - 1, grid_position.1))),
            Self::Horizontal => Some(((grid_position.0, grid_position.1 + 1), (grid_position.0, grid_position.1 - 1))),
            Self::NorthEast => Some(((grid_position.0 + 1, grid_position.1), (grid_position.0, grid_position.1 + 1))),
            Self::NorthWest => Some(((grid_position.0 + 1, grid_position.1), (grid_position.0, grid_position.1 - 1))),
            Self::SouthEast => Some(((grid_position.0 - 1, grid_position.1), (grid_position.0, grid_position.1 + 1))),
            Self::SouthWest => Some(((grid_position.0 - 1, grid_position.1), (grid_position.0, grid_position.1 - 1))),
            Self::Ground => Some((*grid_position, *grid_position)),
            Self::Start => None,

        }
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Pipe>>);
impl Grid {
    pub fn new(lines: Vec<String>) -> Result<Self, ParseInputError> {       
        let parse_line = |line: String| -> Result<Vec<Pipe>, ParseInputError> { line.chars().map(|c| Pipe::try_from(c)).collect() };          
        let pipes: Vec<Vec<Pipe>> = lines.into_iter().map(parse_line).collect::<Result<Vec<Vec<Pipe>>, ParseInputError>>()?;
        Ok(Grid(pipes))
    }

    pub fn get_start_position(&self) -> Position {
        let start_position_in_row = |row: &Vec<Pipe>| -> Option<usize> {
            row.into_iter().position(|pipe| *pipe == Pipe::Start)
        };
        self.0.iter().enumerate().filter_map(|(i, row)| {
            match start_position_in_row(row) {
                Some(column_index) => Some((i, column_index)), 
                None => None
            }
        }).nth(0).unwrap()
    }

    pub fn get_cycle_positions(&self) -> Vec<(usize, usize)> {
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