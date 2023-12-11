use std::{cmp, str, collections, iter};

use itertools::Itertools;

use crate::ParseInputError;

type Cards = [char; 5];

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}  
impl Type {
    fn determine_type(cards: Cards) -> Self {
        let count_frequency = |card: char| -> usize { cards.into_iter().filter(|x: &char| *x == card).count() };
        match collections::HashSet::from(cards).into_iter().map(count_frequency).sorted().collect::<Vec<usize>>()[..] {
            [5] => Type::FiveOfAKind,
            [4] => Type::FourOfAKind,
            [3, 2] => Type::FullHouse,
            [3, 1, 1] => Type::ThreeOfAKind,
            [2, 2, 1] => Type::TwoPair,
            [2, 1, 1, 1, 1] => Type::OnePair,
            _ => Type::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    value: Cards,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.get_type()).cmp(other.get_type()) {
            cmp::Ordering::Less => cmp::Ordering::Greater,          
            cmp::Ordering::Equal => {
                // get the first chars that are unique
                // compare their values
            },
            cmp::Ordering::Greater => cmp::Ordering::Less,
        }
    }
}

impl str::FromStr for Hand {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_whitespace().collect_tuple::<(&str, &str)>().ok_or(ParseInputError {details: "Hand and bid are not delimited by ':'".to_string()})?;
        let value: [char; 5] = hand.chars().collect::<Vec<char>>().try_into().map_err(|_| ParseInputError{ details: "Hand does not consist of exactly 5 cards".to_string() })?;
        let bid = bid.parse::<u32>()?;
        Ok(Hand { value, bid })
    }
}

impl Hand {
    fn get_type(&self) -> Type {
        Type::determine_type(self.value)
    }
}

fn winnings(hands: Vec<Hand>) -> u32{
    let sorted_hands: Vec<Hand> = hands.into_iter().sorted_by(|hand_a, hand_b| hand_a.cmp(hand_b)).collect();
    sorted_hands.into_iter().enumerate().map(|(i, hand)| (i as u32 + 1) * hand.bid).sum()
}

pub fn solve(lines: Vec<String>) {
    
}