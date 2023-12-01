use lazy_static::lazy_static;
use regex::{Regex, Replacer, Captures};

fn convert_digit_name(name: String) -> Option<u8> {
    match name.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    }
}

struct DigitConverter;
impl Replacer for DigitConverter {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let converted_digit_str = match convert_digit_name(caps[0].to_string()) {
            Some(x) => x.to_string(),
            None => caps[0].to_string()
        };
        dst.push_str(&converted_digit_str);
    }
}

fn update_digit_names(line: String) -> String {
    lazy_static! {
        static ref RE_DIGIT: Regex = Regex::new(r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
    }
    RE_DIGIT.replace_all(&line, DigitConverter).to_string()
}

fn capture_digits(line: String) -> Option<(u32, u32)> {
    let digits: Vec<u32> = line.chars().filter_map(|c: char| c.to_digit(10)).collect();
    match (digits.first(), digits.last()) {
        (Some(x), Some(y)) => Some((*x, *y)),
        _ => None,
    }
}

fn concat_digits(digits: Vec<u32>) -> Option<u32> {
    let num_str = digits.into_iter().fold("".to_string(), |acc, x| acc + &x.to_string());
    num_str.parse::<u32>().ok()
}

fn collect_calibration_values(line: String) -> Option<u32> {
    match capture_digits(line) {
        Some((x, y)) => concat_digits(vec![x, y]),
        None => None,
    }
}

pub fn solve(lines: Vec<String>) {
    let lines: Vec<String> = lines.into_iter().map(update_digit_names).collect();
    let cal_values: Vec<u32> = lines.into_iter().filter_map(collect_calibration_values).collect();
    let total_val: u32 = cal_values.into_iter().sum();
    println!("Final Calibration Value: {total_val}");
}

#[cfg(test)]
mod artistic_calibration {
    use crate::year_2023::day_1::artistic_calibration::*;
  
    #[test]
    fn test_capture_line_with_2_digits() {
        let line = "1abc2".to_string();
        assert_eq!(capture_digits(line), Some((1, 2)));
    }

    #[test]
    fn test_capture_line_with_consecutive_digits() {
        let line = "43abc".to_string();
        assert_eq!(capture_digits(line), Some((4, 3)));
    }

    #[test]
    fn test_capture_line_with_one_digits() {
        let line = "4abc".to_string();
        assert_eq!(capture_digits(line), Some((4, 4)));
    }

    #[test]
    fn test_capture_line_only_digits() {
        let line = "418023".to_string();
        assert_eq!(capture_digits(line), Some((4, 3)));
    }

    #[test]
    fn test_capture_single_digit() {
        assert_eq!(capture_digits("4".to_string()), Some((4, 4)));
    }

    #[test]
    fn test_concat_2_digits() {
        assert_eq!(concat_digits(vec![4, 3]), Some(43));
    }

    #[test]
    fn test_concat_3_digits() {
        assert_eq!(concat_digits(vec![4, 3, 4]), Some(434));
    }

    #[test]
    fn test_collect_calibration_values() {
        let line = "pqr3stu8vwx".to_string();
        assert_eq!(collect_calibration_values(line), Some(38));
    }

    #[test]
    fn test_collect_calibration_values_single_digit() {
        let line = "treb7uchet".to_string();
        assert_eq!(collect_calibration_values(line), Some(77));
    }
    
    #[test]
    fn test_collect_calibration_values_consecutive_digit() {
        let line = "47dhax0".to_string();
        assert_eq!(collect_calibration_values(line), Some(40));
    }

    #[test]
    fn test_collect_calibration_with_no_digits() {
        assert_eq!(collect_calibration_values("abcxyz".to_string()), None);
    }
    
    #[test]
    fn test_update_digit_names() {
        let line = "62jfjdsklvnqthree8".to_string();
        assert_eq!(update_digit_names(line), "62jfjdsklvnq38");
    }

    #[test]
    fn test_update_digit_names_two_digit_str() {
        let line = "62jffoursklvnqthree8".to_string();
        assert_eq!(update_digit_names(line), "62jf4sklvnq38");
    }

    #[test]
    fn test_update_digit_names_only_digit_str() {
        assert_eq!(update_digit_names("eight".to_owned()), "8");
    }

    #[test]
    fn test_update_digit_names_consec_digit_str() {
        assert_eq!(update_digit_names("eightninetwothree".to_owned()), "8923");
    }
}