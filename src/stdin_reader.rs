use std::io::{BufRead, Write, self};
use std::num::ParseIntError;

pub fn stdin_reader() -> Box<dyn BufRead> {
    Box::new(io::stdin().lock()) as Box<dyn BufRead>
}

fn stdin_read_integer<R: BufRead>(mut reader: R) -> Result<u32, ParseIntError> {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().parse()
}

pub fn get_day<R: BufRead>(reader: R) -> u8 {
    print!("Please enter a day: ");
    let _ = io::stdout().flush();
    match stdin_read_integer(reader) {
        Ok(n) => n as u8,
        Err(_) => 1,
    } 
}

pub fn get_year<R: BufRead>(reader: R) -> u32 {
    print!("Please enter a year: ");
    let _ = io::stdout().flush();
    match stdin_read_integer(reader) {
        Ok(n) => n,
        Err(_) => 2022,
    } 
}

#[cfg(test)] 
mod main {
    use std::num::ParseIntError;

    use crate::*;
    use crate::stdin_reader::*;

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
        assert_eq!(stdin_read_integer(input), Ok(2));
        assert_eq!(get_day(input), 2);
        assert_eq!(get_year(input), 2);
    }

    #[test]
    fn read_integer_with_spaces() {
        let input = &b"         2    "[..];
        assert_eq!(stdin_read_integer(input), Ok(2));
        assert_eq!(get_day(input), 2);
        assert_eq!(get_year(input), 2);
    }

    #[test]
    fn read_negative_integer() {
        let input = &b"-2"[..];
        let inputted_int: Result<u32, ParseIntError> = stdin_read_integer(input);
        assert!(inputted_int.is_err());
        assert_eq!(get_day(input), 1);
        assert_eq!(get_year(input), 2022);
    }

    #[test]
    fn read_integer_with_alphabetical_chars() {
        let input = &b"abc123"[..];
        let inputted_int: Result<u32, ParseIntError> = stdin_read_integer(input);
        assert!(inputted_int.is_err());
        assert_eq!(get_day(input), 1);
        assert_eq!(get_year(input), 2022);
    }

    #[test]
    fn read_integer_with_no_input() {
        let input = &b""[..];
        let inputted_int: Result<u32, ParseIntError> = stdin_read_integer(input);
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