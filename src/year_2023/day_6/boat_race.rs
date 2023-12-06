use crate::ParseInputError;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn winning_ranges(&self) -> (u32, u32) {
        let discriminant: f32 = (self.time.pow(2) - 4 * self.distance) as f32;
        let root_a = (-1.0 * self.time as f32 + discriminant.sqrt()) / -2.0;
        let root_b = (-1.0 * self.time as f32 - discriminant.sqrt()) / -2.0; 
        (root_a.ceil() as u32, root_b.floor() as u32)
    }

    fn win_count(&self) -> u32 {
        let (a, b) = self.winning_ranges();
        b - a
    }
}

fn parse_line(line: String) -> Result<Vec<u32>, ParseInputError> {
    let digits: &str = line.split(':').nth(1).ok_or(ParseInputError { details: "Nothing delimiting the header from the distances".to_string() })?;
    match digits.split_whitespace().map(|n: &str| n.parse::<u32>()).collect() {
        Ok(ns) => Ok(ns), 
        Err(_) => Err(ParseInputError { details: "Could not parse the digits in a line".to_string()} ),
    }
}

fn parse(lines: (String, String)) -> Result<Vec<Race>, ParseInputError> {
    let (times, distances): (Vec<u32>, Vec<u32>) = (parse_line(lines.0)?, parse_line(lines.1)?);
    Ok(times.into_iter().zip(distances).map(|(time, distance)| Race {time, distance}).collect())
}

pub fn solve(lines: Vec<String>) {
    if let Ok(races) = parse((lines.get(0).unwrap().clone(), lines.get(1).unwrap().clone())) {
        let error_margin: u32 = races.into_iter().map(|race| race.win_count()).fold(1, |acc: u32, x: u32| acc * x);
        println!("Error margin: {error_margin}");
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
        assert_eq!(parse(input), Ok(vec![Race{ time: 7, distance: 9 }, Race{ time: 15, distance: 40 }, Race{ time: 30, distance: 200 }]));
    }

    #[test]
    fn test_winning_ranges_are_both_positive() {

    }

    #[test]
    fn test_winning_ranges_with_one_negative() {

    }

    #[test]
    fn test_winning_ranges_negative_discriminat() {

    }
}