use lazy_static::lazy_static;
use std::str::FromStr;

use crate::ParseInputError;

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref GAME_DATA: regex::Regex = regex::Regex::new(r"^Card (\d+): ([0-9 ]*) \| ([0-9 ]*)$").unwrap();
        }

        let parse_vector_of_numbers = |s: &str| -> Result<Vec<u32>, _> {
            s.split_whitespace().map(|n| n.parse::<u32>()).collect()
        };

        let caps = GAME_DATA.captures(s).ok_or(regex::Error::Syntax("Card line did not match expected pattern".to_string()))?;

        let id: u32 = caps.get(1).unwrap().as_str().parse::<u32>()?;
        let winning_numbers: Vec<u32> = parse_vector_of_numbers(caps.get(2).unwrap().as_str())?;
        let my_numbers: Vec<u32> = parse_vector_of_numbers(caps.get(3).unwrap().as_str())?;

        Ok(Card { id, winning_numbers, my_numbers })
    }
}

pub fn solve(lines: Vec<String>) {}

#[cfg(test)]
mod scratchcards {
    use super::*;

    #[test]
    fn test_parse_card() {
        let line: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();
        assert_eq!(
            line.parse::<Card>().unwrap(),
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn test_parse_card_no_bar() {
        let line: String = "Card 1: 41 48 83 86 17 83 86  6 31 17  9 48 53".to_string();
        assert_eq!(line.parse::<Card>().unwrap_err(), ParseInputError {details: "Card line did not match expected pattern".to_string()});
    }
}
