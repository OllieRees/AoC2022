use crate::ParseInputError;

fn parse_history(line: String) -> Result<Vec<i32>, ParseInputError> {
    line.split_whitespace().map(|x| x.parse::<i32>().map_err(|_| ParseInputError {details: format!("Can't parse {x} to i32")} )).collect()
}

fn predict_next_value(history: &Vec<i32>) -> i32 {
    if history.iter().all(|x| *x == 0) {
        0
    } else {
        history.last().unwrap_or(&0) + predict_next_value(&history.windows(2).map(|adjs| adjs[1] - adjs[0]).collect())
    }
}

fn predict_previous_value(history: &Vec<i32>) -> i32 {
    if history.iter().all(|x| *x == 0) {
        0
    } else {
        history.first().unwrap_or(&0) - predict_previous_value(&history.windows(2).map(|adjs| adjs[1] - adjs[0]).collect())
    }
}

pub fn solve(lines: Vec<String>) {
    if let Ok(history) = lines.into_iter().map(parse_history).collect::<Result<Vec<Vec<i32>>, _>>() {
        let next_value_sum: i32 = history.iter().map(|row| predict_next_value(row)).sum();
        println!("Sum of Next Values {next_value_sum}");
        let prev_value_sum: i32 = history.iter().map(|row| predict_previous_value(row)).sum();
        println!("Sum of Previous Values {prev_value_sum}");
    }
}

#[cfg(test)]
mod mirage_maintenance {
    use super::*;

    #[test]
    fn parse_history_with_varying_spaces() {
        let line: String = "1   2    3 4 5     6".to_string();
        assert_eq!(parse_history(line), Ok(vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn parse_history_with_number_larger_than_9() {
        let line: String = "3 123".to_string();
        assert_eq!(parse_history(line), Ok(vec![3, 123]));
    }

    #[test]
    fn parse_history_with_negative_number() {
        let line: String = "1 -3".to_string();
        assert_eq!(parse_history(line), Ok(vec![1, -3]));
    }

    #[test]
    fn parse_history_with_unparseable_char() {
        let line: String = "1 2a 3".to_string();
        assert!(parse_history(line).is_err());
    }

    #[test]
    fn test_predict_next_value() {
        let history: &Vec<i32> = &vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict_next_value(history), 18);
        let history: &Vec<i32> = &vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict_next_value(history), 28);
        let history: &Vec<i32> = &vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_next_value(history), 68);
    }

    #[test]
    fn test_predict_previous_value() {
        let history: &Vec<i32> = &vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_previous_value(history), 5);
    }
}