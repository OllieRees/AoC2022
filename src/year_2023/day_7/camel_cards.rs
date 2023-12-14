use std::{cmp, str, collections::{self, HashMap}};

use itertools::Itertools;

use crate::ParseInputError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Card { Ace, King, Queen, Jack, Tens, Digit (u8), Joker }
type Cards = [Card; 5];
impl Card {
    fn try_parse(value: char, j_card: Card) -> Result<Self, ParseInputError> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(j_card),
            'T' => Ok(Card::Tens),
            _ => {
                match value.to_digit(10) {
                    Some(x) => Ok(Card::Digit(x as u8)),
                    None => Err(ParseInputError { details: format!("Could not parse {value} into digit") })
                }
            }
        }
    }
}
impl Into<u8> for Card {
    fn into(self) -> u8 {
        match self { Card::Joker => 1, Card::Digit(x) => x, Card::Tens => 10, Card::Jack => 11, Card::Queen => 12, Card::King => 13, Card::Ace => 14, } 
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let self_val: u8 = (*self).into();
        let other_val: u8 = (*other).into();
        self_val.cmp(&other_val)
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)] 
enum Type { FiveOfAKind = 7, FourOfAKind = 6, FullHouse = 5, ThreeOfAKind = 4, TwoPair = 3, OnePair = 2, HighCard = 1 }  
impl Type {
    fn determine_type(cards: Cards) -> Self {
        let mut frequencies: collections::HashMap<usize, usize> = HashMap::from_iter([(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]);
        for card in collections::HashSet::from(cards).into_iter().filter(|card: &Card| *card != Card::Joker) {
            *frequencies.entry(cards.into_iter().filter(|x: &Card| *x == card).count()).or_insert(0) += 1;
        }
        let mut hand_type: Self = match frequencies {
            _ if *frequencies.get(&5).unwrap() == 1 => Self::FiveOfAKind,
            _ if *frequencies.get(&4).unwrap() == 1 => Self::FourOfAKind,
            _ if *frequencies.get(&3).unwrap() == 1 && *frequencies.get(&2).unwrap() == 1 => Self::FullHouse,
            _ if *frequencies.get(&3).unwrap() == 1 => Self::ThreeOfAKind,
            _ if *frequencies.get(&2).unwrap() == 2 => Self::TwoPair,
            _ if *frequencies.get(&2).unwrap() == 1 => Self::OnePair,
            _ => Self::HighCard
        };

        for _ in 0..cards.into_iter().filter(|x: &Card| *x == Card::Joker).count() {
            hand_type = match hand_type {
                Self::HighCard => Self::OnePair,
                Self::OnePair => Self::ThreeOfAKind,
                Self::TwoPair => Self::FullHouse,
                Self::ThreeOfAKind => Self::FourOfAKind,
                Self::FullHouse => Self::FourOfAKind,
                Self::FourOfAKind => Self::FiveOfAKind,
                _ => hand_type,
            }
        }
        hand_type
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    value: Cards,
    bid: u32,
}
impl Hand {
    fn get_type(&self) -> Type {
        Type::determine_type(self.value)
    }

    fn parse(s: &str, j_card: Card) -> Result<Self, ParseInputError> {
        let (hand, bid) = s.split_whitespace().collect_tuple::<(&str, &str)>().ok_or(ParseInputError {details: "Hand and bid are not delimited by ':'".to_string()})?;
        let value: Cards = hand.chars().map(|c| Card::try_parse(c, j_card)).collect::<Result<Vec<Card>, _>>()?.try_into().map_err(|_| ParseInputError{ details: "Hand does not consist of exactly 5 cards".to_string() })?;
        let bid = bid.parse::<u32>()?;
        Ok(Hand { value, bid })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.get_type()).cmp(&other.get_type()) {
            cmp::Ordering::Less => cmp::Ordering::Less,          
            cmp::Ordering::Equal => {
                match self.value.into_iter().zip(other.value).skip_while(|(a, b)| a == b).nth(0) {
                    Some((a, b)) => a.cmp(&b),
                    None => cmp::Ordering::Equal
                }
            },
            cmp::Ordering::Greater => cmp::Ordering::Greater,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn winnings(hands: Vec<Hand>) -> u32{
    let sorted_hands: Vec<Hand> = hands.into_iter().sorted().collect();
    sorted_hands.into_iter().enumerate().map(|(i, hand)| (i as u32 + 1) * hand.bid).sum()
}

pub fn solve(lines: Vec<String>) {
    if let Ok(hands) = lines.iter().map(|line| Hand::parse(&line, Card::Jack)).collect::<Result<Vec<Hand>, _>>() {
        println!("Total Winnings: {}", winnings(hands));
    }
    if let Ok(hands) = lines.iter().map(|line| Hand::parse(&line, Card::Joker)).collect::<Result<Vec<Hand>, _>>() {
        println!("Total Winnings: {}", winnings(hands));
    }
}

#[cfg(test)]
mod camel_cards {
    use super::*;

    #[test]
    fn test_parsing_card() {
        assert_eq!(Card::try_parse('A', Card::Joker), Ok(Card::Ace));
        assert_eq!(Card::try_parse('2', Card::Joker), Ok(Card::Digit(2)));
    }

    #[test]
    fn test_parsing_j_card() {
        assert_eq!(Card::try_parse('J', Card::Jack), Ok(Card::Jack));
        assert_eq!(Card::try_parse('J', Card::Joker), Ok(Card::Joker));
    }

    #[test]
    fn test_parsing_card_with_unmatched_char() {
        assert!(Card::try_parse('I', Card::Joker).is_err());
    }

    #[test]
    fn test_card_order() {
        assert!(Card::Ace == Card::Ace);
        assert!(Card::Ace > Card::King);
        assert!(Card::Jack < Card::Queen);
        assert!(Card::Digit(9) > Card::Digit(8));
        assert!(Card::Digit(2) > Card::Joker);
    }

    #[test]
    fn test_determine_type() {
        let cards: Cards = [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace];
        assert_eq!(Type::determine_type(cards), Type::FiveOfAKind);
        let cards: Cards = [Card::Ace, Card::Ace, Card::Digit(2), Card::Ace, Card::Ace];
        assert_eq!(Type::determine_type(cards), Type::FourOfAKind);
        let cards: Cards = [Card::Ace, Card::Ace, Card::Digit(2), Card::Ace, Card::Jack];
        assert_eq!(Type::determine_type(cards), Type::ThreeOfAKind);
        let cards: Cards = [Card::Ace, Card::Ace, Card::Digit(5), Card::Ace, Card::Digit(5)];
        assert_eq!(Type::determine_type(cards), Type::FullHouse);
        let cards: Cards = [Card::Jack, Card::Ace, Card::Digit(5), Card::Ace, Card::Digit(5)];
        assert_eq!(Type::determine_type(cards), Type::TwoPair);
        let cards: Cards = [Card::Jack, Card::Digit(4), Card::Digit(5), Card::Ace, Card::Digit(5)];
        assert_eq!(Type::determine_type(cards), Type::OnePair);
        let cards: Cards = [Card::Jack, Card::Digit(4), Card::Digit(8), Card::Ace, Card::Digit(5)];
        assert_eq!(Type::determine_type(cards), Type::HighCard);
    }

    #[test]
    fn test_determine_type_joker() {
        let cards: Cards = [Card::Digit(5), Card::Digit(5), Card::Digit(5), Card::Joker, Card::Digit(5)];
        assert_eq!(Type::determine_type(cards), Type::FiveOfAKind);
        let cards: Cards = [Card::Tens, Card::Digit(5), Card::Digit(5), Card::Joker, Card::Digit(5)];
        assert_eq!(Type::determine_type(cards), Type::FourOfAKind);
        let cards: Cards = [Card::King, Card::Tens, Card::Joker, Card::Joker, Card::Tens];
        assert_eq!(Type::determine_type(cards), Type::FourOfAKind);
        let cards: Cards = [Card::King, Card::Joker, Card::Joker, Card::Digit(7), Card::Tens];
        assert_eq!(Type::determine_type(cards), Type::ThreeOfAKind);
        let cards: Cards = [Card::Digit(7), Card::Tens, Card::Joker, Card::Digit(7), Card::Tens];
        assert_eq!(Type::determine_type(cards), Type::FullHouse);
        let cards: Cards = [Card::King, Card::Digit(3), Card::Joker, Card::Digit(7), Card::Tens];
        assert_eq!(Type::determine_type(cards), Type::OnePair);
    }

    #[test]
    fn test_five_jokers() {
        let cards: Cards = [Card::Joker, Card::Joker, Card::Joker, Card::Joker, Card::Joker];
        assert_eq!(Type::determine_type(cards), Type::FiveOfAKind);
    }

    #[test]
    fn test_type_comparison() {
        assert!(Type::ThreeOfAKind > Type::TwoPair);
        assert!(Type::TwoPair > Type::OnePair);
    }

    #[test]
    fn test_compare_hand_equal_type() {
        let hand_a = Hand { value: [Card::Tens, Card::Digit(5), Card::Digit(5), Card::Jack, Card::Digit(5)], bid: 684 };
        let hand_b = Hand { value: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace], bid: 483 };
        assert!(hand_b > hand_a);
    }
    
    #[test]
    fn test_compare_almost_same_hand() {
        let hand_a = Hand { value: [Card::Tens, Card::Digit(7), Card::Digit(7), Card::Jack, Card::Digit(5)], bid: 0 };
        let hand_b = Hand { value: [Card::Tens, Card::Digit(7), Card::Digit(7), Card::Jack, Card::Digit(6)], bid: 0 };
        assert!(hand_b > hand_a);
    }

    #[test]
    fn test_compare_hand_different_type() {
        let hand_a = Hand { value: [Card::Tens, Card::Jack, Card::Digit(5), Card::Jack, Card::Digit(5)], bid: 0 };
        let hand_b = Hand { value: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace], bid: 0 };
        assert!(hand_b > hand_a);
        let hand_a = Hand { value: [Card::King, Card::Tens, Card::Jack, Card::Jack, Card::Tens], bid: 0 };
        let hand_b = Hand { value: [Card::King, Card::King, Card::Digit(6), Card::Digit(7), Card::Digit(7)], bid: 0 };
        assert!(hand_b > hand_a);
    }

    #[test]
    fn test_compare_same_hands() {
        let hand = Hand { value: [Card::Tens, Card::Digit(7), Card::Digit(5), Card::Jack, Card::Digit(5)], bid: 0 };
        assert!(hand == hand);
    }
}