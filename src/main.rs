use std::{fs, fmt::Display, error::Error, num::ParseIntError};

mod input;
use input::*;

mod year_2022;
mod year_2023;

#[derive(Debug, PartialEq, Eq)]
struct ParseInputError {
    details: String,
}

impl Display for ParseInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error: {}", {self.details.clone()})
    }
}

impl Error for ParseInputError {
    fn description(&self) -> &str {
        self.details.as_str()
    }
}

impl From<ParseIntError> for ParseInputError {
    fn from(value: ParseIntError) -> Self {
        ParseInputError { details: value.to_string() }
    }
}

impl From<regex::Error> for ParseInputError {
    fn from(value: regex::Error) -> Self {
        ParseInputError { details: value.to_string() }
    }
}

enum AnswerMode {
    Real,
}

fn get_file_path(solution_type: AnswerMode, year: u32, day: u8) -> String {
    match solution_type {
        AnswerMode::Real => format!("inputs/{year}/{day}/real.txt").to_owned(),
    }
}

fn read_problem_input_file(filepath: String) -> Vec<String> {
    let contents: String = fs::read_to_string(filepath).unwrap();
    contents.split("\n").map(|line| line.to_owned()).collect()
}

fn get_module(year: u32, day: u8) -> Option<impl Fn(Vec<String>)> {
    match year {
        2022 => {
            use crate::year_2022::*;
            match day {
                1 => Some(day_1::calorie_count::solve as fn(Vec<String>)),
                2 => Some(day_2::rps::solve),
                3 => Some(day_3::rucksack::solve),
                4 => Some(day_4::camp_cleanup::solve),
                5 => Some(day_5::supply_stacks::solve),
                6 => Some(day_6::tuning_trouble::solve),
                _ => None,
            }
        },
        2023 => {
            use crate::year_2023::*;
            match day {
                1 => Some(day_1::artistic_calibration::solve),
                2 => Some(day_2::cube_conundrum::solve),
                3 => Some(day_3::gear_ratio::solve),
                4 => Some(day_4::scratchcards::solve),
                5 => Some(day_5::planting_seeds::solve),
                6 => Some(day_6::boat_race::solve),
                7 => Some(day_7::camel_cards::solve),
                8 => Some(day_8::wasteland_traversal::solve),
                9 => Some(day_9::mirage_maintenance::solve),
                10 => Some(day_10::pipe_maze::solve),
                11 => Some(day_11::cosmic_expansion::solve),
                12 => Some(day_12::hot_springs::solve),
                _ => None,
            }
        }
        _ => None,
    }
}

fn main() {
    let year: u32 = get_year(stdin_reader());
    let day: u8 = get_day(stdin_reader());
    if let Some(solver) = get_module(year, day) { 
        let input_file_path = | solution_type: AnswerMode | get_file_path(solution_type, year, day); 
        println!("Reading answers for day {day} in {year}");
        solver(read_problem_input_file(input_file_path(AnswerMode::Real)));
    } else {
        println!("Solution for day {day} in {year} doesn't exist");
    }
}

#[cfg(test)]
mod main {
    use crate::*;
    
    #[test]
    fn real_input_folder_formats_correctly() {
        assert_eq!(get_file_path(AnswerMode::Real, 2023, 3), "inputs/2023/3/real.txt");
    }

    #[test]
    fn read_input() {
        let filepath: String = "src/mocks/problem_input.txt".to_owned();
        assert_eq!(read_problem_input_file(filepath).get(0), Some(&"1000".to_owned()));
    }

    #[test]
    fn read_input_newline() {
        let filepath: String = "src/mocks/problem_input.txt".to_owned();
        assert_eq!(read_problem_input_file(filepath).get(3), Some(&"".to_owned()));
    }
}
