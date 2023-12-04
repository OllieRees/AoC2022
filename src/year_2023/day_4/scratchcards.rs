use lazy_static::lazy_static;
use std::{str::FromStr, collections::HashSet};

use crate::ParseInputError;

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn count_winning_numbers(&self) -> u32 {
        let my_numbers: HashSet<u32> = HashSet::from_iter(self.my_numbers.clone());
        let winning_numbers: HashSet<u32> = HashSet::from_iter(self.winning_numbers.clone());
        my_numbers.intersection(&winning_numbers).count() as u32
    }

    fn point(&self) -> u32 {
        let count: u32 = self.count_winning_numbers();
        match count {
            0 => 0,
            _ => u32::pow(2, count - 1)
        }
    }

    fn cards_ids_won(&self) -> Vec<u32> {
        ((self.id + 1)..=(self.id+self.count_winning_numbers())).collect()
    }
}

impl FromStr for Card {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref GAME_DATA: regex::Regex = regex::Regex::new(r"^Card\s+(\d+): ([0-9 ]+)\s+\|\s+([0-9 ]+)$").unwrap();
        }

        let parse_vector_of_numbers = |s: &str| -> Result<Vec<u32>, _> {
            s.split_whitespace().map(|n| n.parse::<u32>()).collect()
        };
    
        let caps = GAME_DATA.captures(s).ok_or(regex::Error::Syntax(format!("Card did not match expected pattern: {}", s)))?;

        let id: u32 = caps.get(1).unwrap().as_str().parse::<u32>()?;
        let winning_numbers: Vec<u32> = parse_vector_of_numbers(caps.get(2).unwrap().as_str())?;
        let my_numbers: Vec<u32> = parse_vector_of_numbers(caps.get(3).unwrap().as_str())?;

        Ok(Card { id, winning_numbers, my_numbers })
    }
}

fn copies_won(card: &Card, cards: &Vec<Card>) -> u32 {
    let cards_won: Vec<u32> = card.cards_ids_won();
    cards_won.len() as u32 + cards_won.into_iter().map(|card_id: u32| { 
        match cards.into_iter().nth(card_id as usize - 1) {
            Some(card_copy) => copies_won(card_copy, cards),
            None => 0,
        }
    }).sum::<u32>()
}

pub fn solve(lines: Vec<String>) {
    let cards: Result<Vec<Card>, ParseInputError> = lines.iter().map(|line| line.parse::<Card>()).collect();
    if let Ok(cards) = cards {
        let total_points: u32 = cards.iter().map(|card| card.point()).sum();
        println!("Total Points: {total_points}");
        let total_cards_won: u32 = cards.iter().map(|card| 1 + copies_won(card, &cards)).sum();
        println!("Total Cards Won: {total_cards_won}");
    }
}

#[cfg(test)]
mod scratchcards {
    use super::*;

    const EXAMPLE: [&str; 6] = [
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
    "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
    "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
    "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
    "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
    "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ];

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
        assert_eq!(line.parse::<Card>().unwrap_err(), ParseInputError {details: format!("Card did not match expected pattern: {line}")});
    }

    #[test]
    fn test_parse_card_multiple_spaces() {
        let line: String = "Card    1: 41 48 83 86 17   |   83 86  6 31 17  9 48 53".to_string();
        assert_eq!(line.parse::<Card>().unwrap(), Card { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] });
    }

    #[test]
    fn test_winning_number_count() {
        let card = Card { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };
        assert_eq!(card.count_winning_numbers(), 4);
    }

    #[test]
    fn test_no_winning_number_count() {
        let card = Card { id: 5, winning_numbers: vec![87, 83, 26, 28, 32], my_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36] };
        assert_eq!(card.count_winning_numbers(), 0);
    }

    #[test]
    fn test_points() {
        let card = Card { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };
        assert_eq!(card.point(), 8);
    }

    #[test]
    fn test_points_with_no_winning_cards() {
        let card = Card { id: 5, winning_numbers: vec![87, 83, 26, 28, 32], my_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36] };
        assert_eq!(card.point(), 0);
    }

    #[test]
    fn test_card_ids_won() {
        let card = Card { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };
        assert_eq!(card.cards_ids_won(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_card_ids_won_none() {
        let card = Card { id: 5, winning_numbers: vec![87, 83, 26, 28, 32], my_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36] };
        assert_eq!(card.cards_ids_won(), vec![]);
    }

    #[test]
    fn test_card_winnings() {
        let cards: Vec<Card> = EXAMPLE.into_iter().map(|line| line.parse::<Card>().unwrap()).collect();
        assert_eq!(copies_won(cards.get(2).unwrap(), &cards), 3);
    }

    #[test]
    fn test_card_winnings_losing_card() {
        let cards: Vec<Card> = EXAMPLE.into_iter().map(|line| line.parse::<Card>().unwrap()).collect();
        assert_eq!(copies_won(cards.get(4).unwrap(), &cards), 0);
    }
}
