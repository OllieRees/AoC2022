use std::{str::FromStr, collections::HashMap};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    rounds: Vec<Round>, 
}

impl Game {
    fn least_red_count(&self) -> u32 {
        self.rounds.iter().max_by(|x, y| x.red.cmp(&y.red)).unwrap().red
    }
    fn least_blue_count(&self) -> u32 {
        self.rounds.iter().max_by(|x, y| x.blue.cmp(&y.blue)).unwrap().blue
    }
    fn least_green_count(&self) -> u32 {
        self.rounds.iter().max_by(|x, y| x.green.cmp(&y.green)).unwrap().green
    }
    fn power_set(&self) -> u32 {
        self.least_red_count() * self.least_green_count() * self.least_blue_count()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;
impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref GAME_DATA: Regex = Regex::new(r"^Game (\d+): (.*)$").unwrap();
        }
        if let Some(cap) = GAME_DATA.captures(s) {
            let id: u32 = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let rounds: Result<Vec<Round>, _> = cap.get(2).unwrap().as_str().split(";").map(|round_str| round_str.trim().parse::<Round>()).collect();
            return match rounds {
                Ok(rounds) => Ok( Game { id, rounds }),
                Err(_) => Err(ParseGameError),
            };
        }
        Err(ParseGameError)
    }
}

// Not really necessary since the map used when parsing the round can be a map
#[derive(Debug, PartialEq, Eq, Hash)]
enum Colour {
    Red,
    Blue,
    Green,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseColourError;
impl FromStr for Colour {
    type Err = ParseColourError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Colour::Red),
            "blue" => Ok(Colour::Blue),
            "green" => Ok(Colour::Green),
            _ => Err(ParseColourError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRoundError;
impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_colour = | colour: &str| {
            let (count, colour) = colour.split_whitespace().collect_tuple::<(&str, &str)>().unwrap_or(("no number", "no colour"));
            match (count.parse::<u32>(), colour.parse::<Colour>()) {
                (Ok(count), Ok(colour)) => Ok((colour, count)),
                _ => Err(ParseRoundError),
            }
        };
        match s.split(",").map(parse_colour).collect::<Result<Vec<(Colour, u32)>, _>>() {
            Ok(colours) => {
                let colours: HashMap<Colour, u32> = colours.into_iter().collect();
                Ok(Round { red: *colours.get(&Colour::Red).unwrap_or(&0), blue: *colours.get(&Colour::Blue).unwrap_or(&0), green: *colours.get(&Colour::Green).unwrap_or(&0) })
            },
            _ => Err(ParseRoundError),
        }
    }
}

pub fn solve(lines: Vec<String>) {
    if let Ok(games) = lines.into_iter().map(|s| s.parse::<Game>()).collect::<Result<Vec<Game>, ParseGameError>>() {
        let score = games.iter().filter(|game: &&Game|game.least_red_count() <= 12 && game.least_blue_count() <= 14 && game.least_green_count() <= 13).fold(0, |acc, x| acc + x.id);
        println!("ID Sum of Games that could have 12 red, 13 green and 14 blue balls: {score}");
        let score: u32 = games.iter().map(|game| game.power_set()).sum();
        println!("Total Powerset of the Games: {score}");
    }
}

#[cfg(test)]
mod cube_conundrum {
    use crate::year_2023::day_2::cube_conundrum::*;

    #[test]
    fn test_parse_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse::<Game>().unwrap();
        assert_eq!(game, Game { id: 1, rounds: vec![Round{red: 4, blue: 3, green: 0}, Round{red: 1, blue: 6, green: 2}, Round{red: 0, blue: 0, green: 2}] });
    }

    #[test]
    fn test_parse_game_missing_colour() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 6 blue; 2 red".parse::<Game>().unwrap();
        assert_eq!(game, Game { id: 1, rounds: vec![Round{red: 4, blue: 3, green: 0}, Round{red: 1, blue: 6, green: 0}, Round{red: 2, blue: 0, green: 0}] });
    }

    #[test]
    fn test_parse_round() {
        let round = "3 blue, 4 red, 2 green".parse::<Round>().unwrap();
        assert_eq!(round, Round {red: 4, blue: 3, green: 2});
    }

    #[test]
    fn test_parse_round_one_colour() {
        let round = "3 blue".parse::<Round>().unwrap();
        assert_eq!(round, Round {red: 0, blue: 3, green: 0});
    }
}