use itertools::Itertools;
use regex::Regex;

use crate::ParseInputError;


#[derive(Debug, PartialEq, Eq, Clone)]
enum SpringCondition {
    Broken,
    Operational,
    Unknown
}

impl TryFrom<char> for SpringCondition {
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

#[derive(Debug, PartialEq, Eq)]
struct ConditionRecordEntry {
    springs: Vec<SpringCondition>,
    spring_group_size: Vec<usize>
}

impl TryFrom<String> for ConditionRecordEntry {
    type Error = ParseInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new("^(?<springs>[#?.]+) (?<nums>[0-9,]+)$").unwrap();
        let caps = re.captures(value.as_str()).ok_or(ParseInputError {details: format!("Couldn't Capture Record, {}", value)})?;
        Ok(ConditionRecordEntry{
            springs: caps.name("springs").unwrap().as_str().chars().map(|c| SpringCondition::try_from(c)).collect::<Result<Vec<SpringCondition>, ParseInputError>>()?,
            spring_group_size: caps.name("nums").unwrap().as_str().split(',').filter_map(|c| c.to_string().parse::<usize>().ok()).collect()
        })
    }
}

impl ConditionRecordEntry {
    pub fn is_acceptable(&self) -> bool {
         self.springs.split(|x| x == &SpringCondition::Operational).filter(|x| x != &&[]).map(|x| x.len()).collect::<Vec<usize>>() == self.spring_group_size
    }

    pub fn acceptable_permutations(&self) -> impl Iterator<Item=Self> + '_ {
        self.springs.iter().map(|spring| {
            match spring {
                SpringCondition::Unknown => vec![SpringCondition::Broken, SpringCondition::Operational],
                _ => vec![spring.clone()]
            }
        }).multi_cartesian_product().map(|p| ConditionRecordEntry {springs: p, spring_group_size: self.spring_group_size.clone()}).filter(|entry| entry.is_acceptable())
    }
}


#[derive(Debug)]
struct ConditionRecord {
    entries: Vec<ConditionRecordEntry>
}

impl TryFrom<Vec<String>> for ConditionRecord {
    type Error = ParseInputError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let entries = value.into_iter().filter(|s| s != "").map(|s| ConditionRecordEntry::try_from(s)).collect::<Result<Vec<ConditionRecordEntry>, ParseInputError>>()?;
        Ok(ConditionRecord {entries})
    }
}

impl ConditionRecord {
    pub fn total_number_of_condition_permutations(&self) -> usize {
        self.entries.iter().map(|e| e.acceptable_permutations().count()).sum()
    }
}


pub fn solve(lines: Vec<String>) {
    if let Ok(record) = ConditionRecord::try_from(lines) {
        println!("{}", record.total_number_of_condition_permutations());
    }
}

#[cfg(test)]
mod test_hot_springs {
    use super::{ConditionRecordEntry, SpringCondition};

    #[test]
    fn spring_state_conversion_from_char() {
        assert_eq!(SpringCondition::try_from('#'), Ok(SpringCondition::Broken));
        assert_eq!(SpringCondition::try_from('?'), Ok(SpringCondition::Unknown));
        assert_eq!(SpringCondition::try_from('.'), Ok(SpringCondition::Operational));
        assert!(SpringCondition::try_from('x').is_err());
    }

    #[test]
    fn condition_record_entry_parse() {
        let entry: ConditionRecordEntry = ConditionRecordEntry::try_from("???.### 1,1,3".to_string()).unwrap();
        assert_eq!(
            entry.springs,
            vec![
                SpringCondition::Unknown, SpringCondition::Unknown, SpringCondition::Unknown,
                SpringCondition::Operational,
                SpringCondition::Broken, SpringCondition::Broken, SpringCondition::Broken
            ]
        );
        assert_eq!(entry.spring_group_size, vec![1, 1, 3])
    }

    #[test]
    fn condition_record_entry_double_digit_cardinality() {
        assert_eq!(ConditionRecordEntry::try_from("????#????#?#??#?#??. 5,10".to_string()).unwrap().spring_group_size, vec![5, 10]);
    }

    #[test]
    fn condition_record_entry_parse_fail() {
        assert!(ConditionRecordEntry::try_from("???*.### 1,1,3".to_string()).is_err());
        assert!(ConditionRecordEntry::try_from("??? . ### 1,1,3".to_string()).is_err());
        assert!(ConditionRecordEntry::try_from("???.### 1,1|,3".to_string()).is_err());
        assert!(ConditionRecordEntry::try_from("???.### 1, 1 ,3".to_string()).is_err());
        assert!(ConditionRecordEntry::try_from("???.###-1,1,3".to_string()).is_err());
    }

    #[test]
    fn acceptable_entries() {
        assert!(ConditionRecordEntry::try_from(".#.###.#.###### 1,3,1,6".to_string()).unwrap().is_acceptable());
        assert!(ConditionRecordEntry::try_from(".#.###.#.######. 1,3,1,6".to_string()).unwrap().is_acceptable());
        assert!(ConditionRecordEntry::try_from("#.###.#.###### 1,3,1,6".to_string()).unwrap().is_acceptable());
        assert!(ConditionRecordEntry::try_from(".#...#....###. 1,1,3".to_string()).unwrap().is_acceptable());
        assert!(ConditionRecordEntry::try_from(".#.#?#.#.?????? 1,3,1,6".to_string()).unwrap().is_acceptable());
        assert!(!ConditionRecordEntry::try_from(".##.##.#.###### 1,3,1,6".to_string()).unwrap().is_acceptable());
    }

    #[test]
    fn one_permutation_entry() {
        let perms: Vec<ConditionRecordEntry> = ConditionRecordEntry::try_from("???.### 1,1,3".to_string()).unwrap().acceptable_permutations().collect();
        assert_eq!(perms.len(), 1);
        assert_eq!(perms.get(0).unwrap(), &ConditionRecordEntry::try_from("#.#.### 1,1,3".to_string()).unwrap());
    }

        #[test]
    fn multiple_permutation_entry() {
        let perms: Vec<ConditionRecordEntry> = ConditionRecordEntry::try_from("?###???????? 3,2,1".to_string()).unwrap().acceptable_permutations().collect();
        assert_eq!(perms.len(), 10);
    }
}
