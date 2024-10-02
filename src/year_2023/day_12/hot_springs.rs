use itertools::Itertools;
use regex::Regex;

use crate::ParseInputError;


#[derive(Debug, PartialEq, Eq)]
enum SpringState {
    Broken,
    Operational,
    Unknown
}

impl TryFrom<char> for SpringState {
    type Error = ParseInputError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Broken),
            '.' => Ok(Self::Operational),
            '?' => Ok(Self::Unknown),
            _ => Err(ParseInputError {details: format!("{} is not a known Spring State", value)})
        }
    }
}

struct ConditionRecordEntry {
    springs: Vec<SpringState>,
    failsafe: Vec<usize>
}

impl TryFrom<String> for ConditionRecordEntry {
    type Error = ParseInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new("(?<springs>.*) (?<nums>.*)").unwrap();
        let caps = re.captures(value.as_str()).ok_or(ParseInputError {details: format!("Couldn't Capture Record, {}", value)})?;
        Ok(ConditionRecordEntry{
            springs: caps.name("springs").unwrap().as_str().chars().map(|c| SpringState::try_from(c)).collect::<Result<Vec<SpringState>, ParseInputError>>()?,
            failsafe: caps.name("nums").unwrap().as_str().chars().filter_map(|c| c.to_string().parse::<usize>().ok()).collect()
        })
    }
}

struct ConditionRecord {
    entries: Vec<ConditionRecordEntry>
}

impl TryFrom<Vec<String>> for ConditionRecord {
    type Error = ParseInputError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let entries: Vec<ConditionRecordEntry> = value.into_iter().map(|s| ConditionRecordEntry::try_from(s)).collect::<Result<Vec<ConditionRecordEntry>, ParseInputError>>()?;
        Ok(ConditionRecord {entries})
    }
}


pub fn solve(lines: Vec<String>) {

}

#[cfg(test)]
mod test_hot_springs {
    use super::{ConditionRecordEntry, SpringState};

    #[test]
    fn spring_state_conversion_from_char() {
        assert_eq!(SpringState::try_from('#'), Ok(SpringState::Broken));
        assert_eq!(SpringState::try_from('?'), Ok(SpringState::Unknown));
        assert_eq!(SpringState::try_from('.'), Ok(SpringState::Operational));
        assert!(SpringState::try_from('x').is_err());
    }

    #[test]
    fn condition_record_entry_parse() {
        let entry: ConditionRecordEntry = ConditionRecordEntry::try_from("???.### 1,1,3".to_string()).unwrap();
        assert_eq!(
            entry.springs,
            vec![
                SpringState::Unknown, SpringState::Unknown, SpringState::Unknown,
                SpringState::Operational,
                SpringState::Broken, SpringState::Broken, SpringState::Broken
            ]
        );
        assert_eq!(entry.failsafe, vec![1, 1, 3])

    }
}
