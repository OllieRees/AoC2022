use std::fs;

mod day_1;
use day_1::calorie_count;

mod day_2;
use day_2::rps;

mod day_3;
use day_3::rucksack;

mod day_4;
use day_4::camp_cleanup;

mod day_5;
use day_5::supply_stacks;

mod day_6;
use day_6::tuning_trouble;

mod day_7;
use day_7::file_system;

enum AnswerMode {
    Practice,
    Real,
}

const DAY: u8 = 5;

fn answer_mode_path(mode: AnswerMode) -> String {
    match mode {
        AnswerMode::Practice => "practice".to_owned(),
        AnswerMode::Real => "real".to_owned(),
    }
}

type Solution = fn(Vec<String>);
const FUNCTIONS: [Solution; 7] = [
    calorie_count::main,
    rps::main,
    rucksack::main,
    camp_cleanup::main,
    supply_stacks::main,
    tuning_trouble::main,
    file_system::main,
];

fn read_input(folder: AnswerMode) -> Vec<String> {
    let filepath = format!("../inputs/{}/{DAY}.txt", answer_mode_path(folder));
    println!("{}", filepath);
    let contents: String = fs::read_to_string(filepath).unwrap();
    let contents: String = contents
        .strip_suffix("\n")
        .unwrap_or(contents.as_str())
        .to_owned();

    contents
        .split("\n")
        .map(|line: &str| line.to_owned())
        .collect::<Vec<String>>()
}

pub fn trim_input(input_lines: Vec<String>) -> Vec<String> {
    input_lines
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect()
}

fn get_answers_for_day(folder: AnswerMode) {
    let lines = read_input(folder);
    FUNCTIONS.get(usize::from(DAY - 1)).unwrap()(lines);
}

fn main() {
    println!("\nPRACTICE");
    get_answers_for_day(AnswerMode::Practice);
    println!("\nREAL");
    get_answers_for_day(AnswerMode::Real);
}
