use aho_corasick::AhoCorasick;
use itertools::iproduct;

struct Position {
    start: (usize, usize),
    end: (usize, usize),
}

impl Position {
    fn is_in_position(&self, position: (usize, usize)) -> bool {
        (self.start.0 == position.0 || self.end.0 == position.0) && (self.end.1 >= position.1 && self.start.1 <= position.1) 
    }

    fn adjacent_positions(&self) -> Vec<(usize, usize)> {
        let start_x = if self.start.0 == 0 {self.start.0} else {self.start.0 - 1};
        let start_y = if self.start.1 == 0 {self.start.1} else {self.start.1 - 1};
        iproduct!((start_x..(self.end.0 + 2)), (start_y..(self.end.1 + 2))).filter(|pos: &(usize, usize)| !self.is_in_position(*pos)).collect()
    }
}

struct EnginePart {
    value: u32,
    position: Position
}

impl EnginePart {
    fn is_engine_part(&self, grid: &Vec<String>) -> bool {
        // get adjacent parts
        let adjacent_parts = self.position.adjacent_positions();
        let grid_value = |x: usize, y: usize| -> Option<char> {
            match grid.get(x) {
                Some(row) => match row.char_indices().nth(y) {
                    Some((_, c)) => Some(c),
                    None => None,
                }
                None => None,
            }
        };
        adjacent_parts.into_iter().any(|(x, y)| {
            match grid_value(x, y) {
                Some(val ) => is_symbol(val),
                None => false,
            }
        })
    }
}

fn get_numbers_and_positions(line: &String) -> Vec<(u32, usize, usize)> {
    let number_strs: Vec<&str> = line.split(|c: char| !c.is_digit(10)).filter(|s| !s.is_empty()).collect();
    let ac = AhoCorasick::new(number_strs.clone()).unwrap();
    let mut matches: Vec<(u32, usize, usize)> = vec![];
    for mat in ac.find_iter(&line) {
        let n: u32 = number_strs.get(mat.pattern().as_usize()).unwrap().to_string().parse::<u32>().unwrap();
        matches.push((n, mat.start(), mat.end() - 1));
    }
    matches
}

fn parse_grid(grid: Vec<String>) -> Vec<EnginePart> {
    let mut parts: Vec<EnginePart> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (n, start, end) in get_numbers_and_positions(row).into_iter() {
            let pos: Position = Position { start: (i, start), end: (i, end)}; 
            let part: EnginePart = EnginePart { value: n, position: pos };
            if part.is_engine_part(&grid) { parts.push(part); }
        }
    }
    parts
} 

fn is_symbol(symbol: char) -> bool {
    !(symbol.is_ascii_digit() || symbol == '.')
}

// fn symbol_positions(grid: Vec<String>) -> Vec<(usize, usize)> {
//     let get_symbol_positions = |s: String| -> Vec<usize> {
//         s.char_indices().filter_map(|(c_i, c)| if is_symbol(c) {Some(c_i)} else {None}).collect()
//     };
//     let get_grid_positions_from_row = | (i, s): (usize, String)| -> Vec<(usize, usize)> {
//         iter::repeat(i).zip(get_symbol_positions(s)).collect()
//     };
//     grid.into_iter().enumerate().map(get_grid_positions_from_row).flatten().collect()
// }

fn parse_engine_parts(grid: Vec<String>) -> Vec<u32> {
    parse_grid(grid).into_iter().map(|part| part.value).collect()
}

pub fn solve(lines: Vec<String>) {
    println!("Engine Part Sum: {}", parse_engine_parts(lines).into_iter().sum::<u32>());
}

#[cfg(test)]
mod gear_ratio {
    use crate::year_2023::day_3::gear_ratio::*;

    const EXAMPLE: [&str; 10] = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598.."
    ];

    #[test]
    fn test_parse_engine_part_horizontal() {
        let input: [&str; 1] = ["467#114"];
        assert_eq!(parse_engine_parts(input.map(String::from).to_vec()), vec![467, 114]);
    }

    #[test]
    fn test_parse_engine_part_vertical() {
        let input: [&str; 3] = ["467", ".#.", ".14"];
        assert_eq!(parse_engine_parts(input.map(String::from).to_vec()), vec![467, 14]);
    }

    #[test]
    fn test_parse_engine_part_diagonal() {
        let input: [&str; 3] = [".67", "#..", ".4."];
        assert_eq!(parse_engine_parts(input.map(String::from).to_vec()), vec![67, 4]);
    }

    #[test]
    fn test_no_adjacent_engine_parts() {
        let input: [&str; 3] = ["673...", ".....#", "1234.."];
        assert_eq!(parse_engine_parts(input.map(String::from).to_vec()), vec![]);
    }

    #[test]
    fn test_parse_engine_parts() {
        assert_eq!(parse_engine_parts(EXAMPLE.map(String::from).to_vec()), vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }

    #[test]
    fn is_engine_part() {
        let engine = EnginePart{ value: 467,  position: Position{ start: (0, 0), end: (0, 2) }};
        assert!(engine.is_engine_part(&EXAMPLE.map(String::from).to_vec()));
    }

    #[test]
    fn is_not_engine_part() {
        let engine = EnginePart{ value: 114,  position: Position{ start: (0, 5), end: (0, 7) }};
        assert!(!engine.is_engine_part(&EXAMPLE.map(String::from).to_vec()));
    }
    
    #[test]
    fn test_numbers_and_positions() {
        let line = "467..114..".to_string();
        assert_eq!(get_numbers_and_positions(&line), vec![(467, 0, 2), (114, 5, 7)]);
    }

    #[test]
    fn test_no_numbers_and_positions() {
        let line = "".to_string();
        assert_eq!(get_numbers_and_positions(&line), vec![]);
    }
    
    #[test]
    fn get_adjacent_positions() {
        let position = Position {start: (3, 2), end: (3, 4)};
        assert_eq!(position.adjacent_positions(), vec![(2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (3, 1), (3, 5), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5)]);
    }

    #[test]
    fn get_adjacent_positions_at_edge() {
        let position = Position {start: (0, 2), end: (0, 4)};
        assert_eq!(position.adjacent_positions(), vec![(0, 1), (0, 5), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5)]);
    }

    #[test]
    fn test_is_not_symbol() {
        assert!(is_symbol('#'));
        assert!(is_symbol('+'));
        assert!(is_symbol('$'));
    }

    #[test]
    fn test_number_is_not_symbol() {
        assert!(!is_symbol('9'));
    }

    #[test]
    fn test_dot_is_not_symbol() {
        assert!(!is_symbol('.'));
    }
}