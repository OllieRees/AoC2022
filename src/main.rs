use std::fs;

mod input;
use input::*;

mod year_2022;


enum AnswerMode {
    Practice,
    Real,
}

fn get_file_path(solution_type: AnswerMode, year: u32, day: u8) -> String {
    match solution_type {
        AnswerMode::Practice => format!("inputs/{year}/{day}/practice.txt").to_owned(),
        AnswerMode::Real => format!("inputs/{year}/{day}/real.txt").to_owned(),
    }
}

fn read_problem_input_file(filepath: String) -> Vec<String> {
    let contents: String = fs::read_to_string(filepath).unwrap();
    contents.split("\n").map(|line| line.trim().to_owned()).collect()
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
        _ => None,
    }
}

fn main() {
    let year: u32 = get_year(stdin_reader());
    let day: u8 = get_day(stdin_reader());
    let solver = get_module(year, day).expect(format!("Solution for day {day} in {year} doesn't exist").as_str());
    let input_file_path = | solution_type: AnswerMode | get_file_path(solution_type, year, day); 
    println!("Reading answers for day {day} in {year}");

    println!("Practice Answers: ");
    let input: Vec<String> = read_problem_input_file(input_file_path(AnswerMode::Practice));
    solver(input);

    println!("Real Answers: "); 
    let input = read_problem_input_file(input_file_path(AnswerMode::Real));
    solver(input);
}

#[cfg(test)]
mod main {
    use crate::*;

    #[test]
    fn practice_input_folder_formats_correctly() {
        assert_eq!(get_file_path(AnswerMode::Practice, 2023, 3), "inputs/2023/3/practice.txt");
    }
    
    #[test]
    fn real_input_folder_formats_correctly() {
        assert_eq!(get_file_path(AnswerMode::Real, 2023, 3), "inputs/2023/3/real.txt");
    }

    #[test]
    fn read_input() {
        let filepath: String = "src/mocks/problem_input.txt".to_owned();
        assert_eq!(read_problem_input_file(filepath).get(0), Some(&"1000".to_owned()));
    }
}