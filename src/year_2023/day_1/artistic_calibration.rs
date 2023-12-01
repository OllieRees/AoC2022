use aho_corasick::AhoCorasick;

trait DigitCapture {
    fn capture_digits(line: String) -> Option<(u8, u8)>;
}

struct JustDigits;
impl DigitCapture for JustDigits {
    fn capture_digits(line: String) -> Option<(u8, u8)> {
        let digits: Vec<u32> = line.chars().filter_map(|c: char| c.to_digit(10)).collect();
        match (digits.first(), digits.last()) {
            (Some(x), Some(y)) => Some((*x as u8, *y as u8)),
            _ => None,
        }
    }
}

struct DigitNames;
impl DigitCapture for DigitNames {
    fn capture_digits(line: String) -> Option<(u8, u8)> {
        let re_digit: &Vec<&str> = &vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let digit: &Vec<u8> = &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let ac: AhoCorasick = AhoCorasick::builder().ascii_case_insensitive(true).build(re_digit).unwrap();
        let digits: Vec<u8> = ac.find_overlapping_iter(&line).map(|mat| (digit[mat.pattern().as_usize()])).collect();
        match (digits.first(), digits.last()) {
            (Some(x), Some(y)) => Some((*x, *y)),
            _ => None,
        }
    } 
}

fn concat_digits(digits: (u8, u8)) -> u32 {
    digits.0 as u32 * 10 + digits.1 as u32
}

fn collect_calibration_values<DC: DigitCapture>(line: String, _digit_capturer: &DC) -> Option<u32> {
    match DC::capture_digits(line) {
        Some((x, y)) => Some(concat_digits((x, y))),
        None => None,
    }
}

fn total_calibration_value<DC: DigitCapture>(lines: Vec<String>, digit_capturer: DC) -> u32 {
    let cal_values: Vec<u32> = lines.into_iter().filter_map(|line| collect_calibration_values(line, &digit_capturer)).collect();
    cal_values.into_iter().sum()
}

pub fn solve(lines: Vec<String>) {
    let total_val: u32 = total_calibration_value(lines.clone(), JustDigits);
    println!("Final Calibration value for part 1: {total_val}");
    let total_val: u32 = total_calibration_value(lines.clone(), DigitNames);
    println!("Final Calibration value for part 2: {total_val}");
}

#[cfg(test)]
mod artistic_calibration {
    use crate::year_2023::day_1::artistic_calibration::*;

    #[test]
    fn test_concat_2_digits() {
        assert_eq!(concat_digits((4, 3)), 43);
    }

    #[test]
    fn test_concat_first_digit_0() {
        assert_eq!(concat_digits((0, 3)), 3);
    }

    #[test]
    fn test_collect_calibration_values() {
        let line = "pqr3stu8vwx".to_string();
        assert_eq!(collect_calibration_values(line, &JustDigits), Some(38));
    }

    #[test]
    fn test_collect_calibration_values_single_digit() {
        let line = "treb7uchet".to_string();
        assert_eq!(collect_calibration_values(line, &JustDigits), Some(77));
    }
    
    #[test]
    fn test_collect_calibration_values_consecutive_digit() {
        let line = "47dhax".to_string();
        assert_eq!(collect_calibration_values(line, &JustDigits), Some(47));
    }

    #[test]
    fn test_collect_calibration_with_no_digits() {
        assert_eq!(collect_calibration_values("abcxyz".to_string(), &JustDigits), None);
    }

    #[test]
    fn test_update_digit_names() {
        assert_eq!(collect_calibration_values("62jfjdsklvnqthree8".to_string(), &DigitNames), Some(68));
    }

    #[test]
    fn test_update_digit_names_only_digit_str() {
        assert_eq!(collect_calibration_values("eight".to_string(), &DigitNames), Some(88));
    }

    #[test]
    fn test_update_digit_names_consec_digit_str() {
        assert_eq!(collect_calibration_values("eightninetwothree".to_string(), &DigitNames), Some(83));
    }

    #[test]
    fn test_update_digit_names_blended_digit_str() {
        assert_eq!(collect_calibration_values("five72sevenjf59nineeight".to_string(), &DigitNames), Some(58));
    }
}