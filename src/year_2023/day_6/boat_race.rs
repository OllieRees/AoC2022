use crate::ParseInputError;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_ranges(&self) -> (u64, u64) {
        // solve for x: -x(x - time) - distance >= 0 
        let discriminant: f64 = (self.time.pow(2) - 4 * self.distance) as f64;
    
        let root = |sig: f64| -> f64 {
            let num: f64 = -1.0 * self.time as f64 + sig * discriminant.sqrt();
            num / -2.0
        };

        let mut root_a: f64 = root(1.0);
        let mut root_b: f64 = root(-1.0); // will always be the larger of the two

        if root_a.fract() == 0.0 {
            root_a += 1.0;
        }
        if root_b.fract() == 0.0 {
            root_b -= 1.0;
        }

        (root_a.ceil() as u64, root_b.floor() as u64)
    }

    fn win_count(&self) -> u64 {
        let (a, b) = self.winning_ranges();
        b - a + 1
    }
}

fn parse_line(line: String) -> Result<Vec<u64>, ParseInputError> {
    let digits: &str = line.split(':').nth(1).ok_or(ParseInputError { details: "Nothing delimiting the header from the distances".to_string() })?;
    match digits.split_whitespace().map(|n: &str| n.parse::<u64>()).collect() {
        Ok(ns) => Ok(ns), 
        Err(_) => Err(ParseInputError { details: "Could not parse the digits in a line".to_string()} ),
    }
}

fn parse_races(lines: (String, String)) -> Result<Vec<Race>, ParseInputError> {
    let (times, distances): (Vec<u64>, Vec<u64>) = (parse_line(lines.0)?, parse_line(lines.1)?);
    Ok(times.into_iter().zip(distances).map(|(time, distance)| Race {time, distance}).collect())
}

fn parse_race(lines: (String, String)) -> Result<Race, ParseInputError> {
    let parse_line = |line: String| -> Result<u64, ParseInputError> {
        let digits = line.split(':').nth(1).ok_or(ParseInputError { details: "Nothing delimiting the header from the distances".to_string() })?;
        digits.chars().into_iter().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u64>().map_err(|_| ParseInputError { details: "Failed to parse line into a single digit".to_string() })
    };
    let (time, distance) = (parse_line(lines.0)?, parse_line(lines.1)?);
    Ok(Race { time, distance})
}

pub fn solve(lines: Vec<String>) {
    if let Ok(races) = parse_races((lines.get(0).unwrap().clone(), lines.get(1).unwrap().clone())) {
        let error_margin: u64 = races.iter().map(|race| race.win_count()).fold(1, |acc: u64, x: u64| acc * x);
        println!("Error margin: {error_margin}");
    }
    if let Ok(race) = parse_race((lines.get(0).unwrap().clone(), lines.get(1).unwrap().clone())) {
        println!("Win Count: {}", race.win_count());
    }
}

#[cfg(test)]
pub mod boat_race {
    use super::*;

    #[test]
    fn test_parse_distance() {
        let input = "Distance:  9  40  200".to_string();
        assert_eq!(parse_line(input), Ok(vec![9, 40, 200]));
    }

    #[test]
    fn test_parse_time() {
        let input = "Time:      7  15   30".to_string();
        assert_eq!(parse_line(input), Ok(vec![7, 15, 30]));
    }

    #[test]
    fn test_parse_input() {
        let input = ("Time:      7  15   30".to_string(), "Distance:  9  40  200".to_string());
        assert_eq!(parse_races(input), Ok(vec![Race{ time: 7, distance: 9 }, Race{ time: 15, distance: 40 }, Race{ time: 30, distance: 200 }]));
    }

    #[test]
    fn test_parse_race() {
        let input = ("Time:      7  15   30".to_string(), "Distance:  9  40  200".to_string());
        assert_eq!(parse_race(input), Ok(Race{time: 71530, distance: 940200}));
    }

    #[test]
    fn test_winning_ranges_are_both_positive() {
        let race = Race{ time: 7, distance: 9 };
        assert_eq!(race.winning_ranges(), (2, 5));
        let race = Race{ time: 15, distance: 40 };
        assert_eq!(race.winning_ranges(), (4, 11));
        let race = Race{ time: 30, distance: 200 };
        assert_eq!(race.winning_ranges(), (11, 19));
    }

    #[test]
    fn test_winning_counts() {
        let race = Race{ time: 7, distance: 9 };
        assert_eq!(race.win_count(), 4);
        let race = Race{ time: 15, distance: 40 };
        assert_eq!(race.win_count(), 8);
        let race = Race{ time: 30, distance: 200 };
        assert_eq!(race.win_count(), 9);
    }

    #[test]
    fn test_part_2() {
        let race = Race{ time: 55999793, distance: 401148522741405 };
        assert_eq!(race.win_count(), 4);
    }
}