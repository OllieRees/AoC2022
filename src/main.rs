use std::{fmt, fs};

mod stdin_reader;
use stdin_reader::*;

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