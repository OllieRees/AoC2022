use itertools::Itertools;

use crate::ParseInputError;

fn group_mappings(lines: Vec<String>) -> Result<[Vec<String>; 8], ParseInputError> {
    let lines: Result<[Vec<String>; 8], _> = lines.split(|line| line == "").map(|line| line.to_vec()).collect::<Vec<_>>().try_into();
    match lines {
        Ok(x) => Ok(x),
        _ => Err(ParseInputError{ details: "Input does not have 8 groups".to_string() })
    }
}

fn seed_location(seed: u64, maps: Vec<impl Fn(u64) -> u64>) -> u64 {
    0
}

pub fn solve(lines: Vec<String>) {

}

#[cfg(test)]
pub mod planting_seeds {
    use std::any::TypeId;

    use crate::{year_2023::day_5::planting_seeds::*, read_problem_input_file};
    
    #[test]
    fn test_grouping_input_by_mapping() {
        let example: Vec<String >= read_problem_input_file("inputs/2023/5/practice.txt".to_owned());
        assert_eq!(group_mappings(example), Ok([
            vec!["seeds: 79 14 55 13".to_string()],
            vec!["seed-to-soil map:".to_string(), "50 98 2".to_string(), "52 50 48".to_string()],
            vec!["soil-to-fertilizer map:".to_string(), "0 15 37".to_string(), "37 52 2".to_string(), "39 0 15".to_string()],
            vec!["fertilizer-to-water map:".to_string(), "49 53 8".to_string(), "0 11 42".to_string(), "42 0 7".to_string(), "57 7 4".to_string()],
            vec!["water-to-light map:".to_string(), "88 18 7".to_string(), "18 25 70".to_string()],
            vec!["light-to-temperature map:".to_string(), "45 77 23".to_string(), "81 45 19".to_string(), "68 64 13".to_string()],
            vec!["temperature-to-humidity map:".to_string(), "0 69 1".to_string(), "1 0 69".to_string()],
            vec!["humidity-to-location map:".to_string(), "60 56 37".to_string(), "56 93 4".to_string()]
        ]));
    }

    // #[test]
    // fn test_parse_mapping() {
    //     let input = vec!["soil-to-fertilizer map:".to_string(), "0 15 37".to_string(), "37 52 2".to_string(), "39 0 15".to_string()];
    //     assert_eq!(parse_mapping(input), Ok(vec![(15, 0, 37), (52, 37, 2), (0, 39, 15)]));
    // }

    // #[test]
    // fn test_parse_bad_mapping() {
    //     let input = vec!["soil-to-fertilizer map:".to_string(), "0 15 37 2".to_string()];
    //     assert!(parse_mapping(input).is_err());
    // }

}