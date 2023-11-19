use std::num::ParseIntError;
use std::{fmt, io, fs};
use std::io::{Read, Write, BufRead};

mod day_1;
use day_1::calorie_count;

enum InputFolder {
    Practice {year: u32, day: u8},
    Real {year: u32, day: u8},
}

impl fmt::Display for InputFolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputFolder::Practice { year, day } => write!(f, "inputs/{year}/{day}/practice.txt"),
            InputFolder::Real { year, day } => write!(f, "inputs/{year}/{day}/real.txt"),
        }
    }
}

fn stdin_reader() -> Box<dyn BufRead> {
    Box::new(io::stdin().lock()) as Box<dyn BufRead>
}

fn stdio_read_integer<R: BufRead>(mut reader: R) -> Result<u32, ParseIntError> {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().parse()
}

fn get_day<R: BufRead>(reader: R) -> u8 {
    print!("Please enter a day: ");
    let _ = io::stdout().flush();
    match stdio_read_integer(reader) {
        Ok(n) => n as u8,
        Err(_) => 1,
    } 
}

fn get_year<R: BufRead>(reader: R) -> u32 {
    print!("Please enter a year: ");
    let _ = io::stdout().flush();
    match stdio_read_integer(reader) {
        Ok(n) => n,
        Err(_) => 2022,
    } 
}

fn read_problem_input_file(filepath: String) -> Vec<String> {
    let contents: String = fs::read_to_string(filepath).unwrap();
    contents.split("\n").map(|line| line.trim().to_owned()).collect()
}

fn main() {
    let year: u32 = get_year(stdin_reader());
    let day: u8 = get_day(stdin_reader());
    println!("Reading answers for day {day} in {year}");
    let practice_input = read_problem_input_file(InputFolder::Practice { year, day }.to_string());
    let real_input = read_problem_input_file(InputFolder::Real { year, day }.to_string());
    println!("Practice Answers: ");
    calorie_count::main(practice_input);
    println!("Real Answers: "); 
    calorie_count::main(real_input);
}

#[cfg(test)] 
mod main {
    use std::num::ParseIntError;

    use crate::{stdio_read_integer, InputFolder, get_day, get_year, read_problem_input_file};

    #[test]
    fn practice_input_folder_formats_correctly() {
        let mock_folder = InputFolder::Practice { year: 2022, day: 1 }.to_string();
        assert_eq!(mock_folder, "inputs/2022/1/practice.txt");
    }
    
    #[test]
    fn real_input_folder_formats_correctly() {
        let mock_folder = InputFolder::Real { year: 2022, day: 1 }.to_string();
        assert_eq!(mock_folder, "inputs/2022/1/real.txt");
    }

    #[test]
    fn read_integer() {
        let input = &b"2"[..];
        assert_eq!(stdio_read_integer(input), Ok(2));
        assert_eq!(get_day(input), 2);
        assert_eq!(get_year(input), 2);
    }

    #[test]
    fn read_integer_with_spaces() {
        let input = &b"         2    "[..];
        assert_eq!(stdio_read_integer(input), Ok(2));
        assert_eq!(get_day(input), 2);
        assert_eq!(get_year(input), 2);
    }

    #[test]
    fn read_negative_integer() {
        let input = &b"-2"[..];
        let inputted_int: Result<u32, ParseIntError> = stdio_read_integer(input);
        assert!(inputted_int.is_err());
        assert_eq!(get_day(input), 1);
        assert_eq!(get_year(input), 2022);
    }

    #[test]
    fn read_integer_with_alphabetical_chars() {
        let input = &b"abc123"[..];
        let inputted_int: Result<u32, ParseIntError> = stdio_read_integer(input);
        assert!(inputted_int.is_err());
        assert_eq!(get_day(input), 1);
        assert_eq!(get_year(input), 2022);
    }

    #[test]
    fn read_integer_with_no_input() {
        let input = &b""[..];
        let inputted_int: Result<u32, ParseIntError> = stdio_read_integer(input);
        assert!(inputted_int.is_err());
        assert_eq!(get_day(input), 1);
        assert_eq!(get_year(input), 2022);
    }

    #[test]
    fn read_input() {
        let filepath: String = "src/mocks/problem_input.txt".to_owned();
        assert_eq!(read_problem_input_file(filepath).get(0), Some(&"1000".to_owned()));
    }
}