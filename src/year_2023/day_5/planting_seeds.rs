use itertools::Itertools;

use crate::ParseInputError;

type Range = (u64, u64, u64);

// the actual logic to solve the puzzles
fn is_in_range(x: u64, range: Range) -> bool {
    x >= range.0 && x <= range.0 + range.2
}

fn execute_range(x: u64, range: Range) -> u64 {
    match x < range.0 || range.0 + range.2 < x {
        true => x,
        false => range.1 + (x - range.0),
    }
}
fn seed_location(seed: u64, mappings: &Vec<Vec<Range>>) -> u64 {
    let get_next_value = |seed: u64, ranges: &Vec<Range>| -> u64 {
        match ranges.iter().filter(|range: &&(u64, u64, u64)| is_in_range(seed, **range)).nth(0) {
            Some(range) => {
                execute_range(seed, *range)
            },
            None => seed,
        }
    };
    mappings.into_iter().fold(seed, get_next_value)
}

// all these functions just to parse this input - use regex?
fn group_input(lines: Vec<String>) -> Result<(String, Vec<Vec<String>>), ParseInputError> {
    let groups: Vec<Vec<String>> = lines.split(|line| line == "").map(|line| line.to_vec()).collect::<Vec<_>>();
    let (seeds, mappings) = groups.split_at(1);
    let seeds = seeds.get(0).unwrap().get(0).unwrap().split(":").nth(1).ok_or(ParseInputError { details: "Seeds line is improperly formatted".to_string() })?;
    let mappings: Vec<Vec<String>> = mappings.iter().map(|mapping: &Vec<String>| mapping[1..].to_vec()).collect::<Vec<_>>().try_into().map_err(|_| ParseInputError { details: "There aren't 7 mappings".to_string() } )?;
    Ok((seeds.trim().to_string(), mappings))
}

fn parse_seeds(seeds: String) -> Result<Vec<u64>, ParseInputError> {
    match seeds.split_whitespace().map(|n: &str| n.parse::<u64>()).collect() {
        Ok(ns) => Ok(ns), 
        Err(_) => Err(ParseInputError { details: "Could not parse seeds to a number".to_string()} ),
    }
}

fn parse_mapping(ranges: Vec<String>) -> Result<Vec<Range>, ParseInputError> {
    let parse_range = |range: String| -> Result<Range, ParseInputError> {
        let (dest_start, src_start, range_size): (&str, &str, &str) = range.split_whitespace().collect_tuple().ok_or(
            ParseInputError{details: "Mapping did not have exactly 3 numbers".to_string()}
        )?;
        Ok((src_start.parse::<u64>()?, dest_start.parse::<u64>()?, range_size.parse::<u64>()?))
    };
    ranges.into_iter().map(parse_range).collect()
}

fn parse(lines: Vec<String>) -> Result<(Vec<u64>, Vec<Vec<Range>>), ParseInputError> {
    let (seeds, groups) = group_input(lines)?;
    let groups: Vec<Vec<Range>>  = groups.into_iter().map(parse_mapping).collect::<Result<Vec<Vec<Range>>, ParseInputError>>()?;
    Ok((parse_seeds(seeds)?, groups))
}

pub fn solve(lines: Vec<String>) {
    if let Ok((seeds, mappings)) = parse(lines) {
        println!("Minimum Location {}", seeds.into_iter().map(|seed| seed_location(seed, &mappings)).min().unwrap());
    }
}

#[cfg(test)]
pub mod planting_seeds {
    use crate::{year_2023::day_5::planting_seeds::*, read_problem_input_file};
    
    #[test]
    fn test_grouping_input_by_mapping() {
        let example: Vec<String >= read_problem_input_file("inputs/2023/5/practice.txt".to_owned());
        assert_eq!(parse(example), Ok(
            (vec![79, 14, 55, 13], 
            vec![
                vec![(98, 50, 2), (50, 52, 48)],
                vec![(15, 0, 37), (52, 37, 2), (0, 39, 15)],
                vec![(53, 49, 8), (11, 0, 42), (0, 42, 7), (7, 57, 4)],
                vec![(18, 88, 7), (25, 18, 70)],
                vec![(77, 45, 23), (45, 81, 19), (64, 68, 13)],
                vec![(69, 0, 1), (0, 1, 69)],
                vec![(56, 60, 37), (93, 56, 4)]
            ])
        ));
    }

    #[test]
    fn test_seed_location() {
        let mappings: Vec<Vec<Range>> =      
            vec![
                vec![(98, 50, 2), (50, 52, 48)],
                vec![(15, 0, 37), (52, 37, 2), (0, 39, 15)],
                vec![(53, 49, 8), (11, 0, 42), (0, 42, 7), (7, 57, 4)],
                vec![(18, 88, 7), (25, 18, 70)],
                vec![(77, 45, 23), (45, 81, 19), (64, 68, 13)],
                vec![(69, 0, 1), (0, 1, 69)],
                vec![(56, 60, 37), (93, 56, 4)]
            ];
        assert_eq!(seed_location(79, &mappings), 82);
    }

    #[test]
    fn test_parse_mapping() {
        let input: Vec<String> = vec!["0 15 37".to_string(), "37 52 2".to_string(), "39 0 15".to_string()];
        assert_eq!(parse_mapping(input), Ok(vec![(15, 0, 37), (52, 37, 2), (0, 39, 15)]));
    }

    #[test]
    fn test_parse_bad_mapping() {
        let input: Vec<String> = vec!["0 15 37 2".to_string(), "37 52 2".to_string(), "39 0 15".to_string()];
        assert!(parse_mapping(input).is_err());
    }

}