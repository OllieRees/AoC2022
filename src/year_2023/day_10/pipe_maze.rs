use crate::ParseInputError;

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
struct Grid(Vec<Vec<Pipe>>);
impl Grid {
    pub fn new(lines: Vec<String>) -> Result<Self, ParseInputError> {       
        let parse_line = |line: String| -> Result<Vec<Pipe>, ParseInputError> { line.chars().map(|c| Pipe::try_from(c)).collect() };          
        let pipes: Vec<Vec<Pipe>> = lines.into_iter().map(parse_line).collect::<Result<Vec<Vec<Pipe>>, ParseInputError>>()?;
        Ok(Grid(pipes))
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