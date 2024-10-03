use itertools::intersperse;
use regex::Regex;

use crate::ParseInputError;


#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug)]
struct ConditionRecordEntry {
    springs: Vec<SpringState>,
    broken_cardinalities: Vec<usize>
}

impl TryFrom<String> for ConditionRecordEntry {
    type Error = ParseInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new("^(?<springs>[#?.]+) (?<nums>[0-9,]+)$").unwrap();
        let caps = re.captures(value.as_str()).ok_or(ParseInputError {details: format!("Couldn't Capture Record, {}", value)})?;
        Ok(ConditionRecordEntry{
            springs: caps.name("springs").unwrap().as_str().chars().map(|c| SpringState::try_from(c)).collect::<Result<Vec<SpringState>, ParseInputError>>()?,
            broken_cardinalities: caps.name("nums").unwrap().as_str().chars().filter_map(|c| c.to_string().parse::<usize>().ok()).collect()
        })
    }
}

impl ConditionRecordEntry {
    pub fn is_acceptable_entry(&self) -> bool {
         self.springs.split(|x| x == &SpringState::Operational).filter(|x| x != &&[]).map(|x| x.len()).collect::<Vec<usize>>() == self.broken_cardinalities
    }

    fn known_accepted_springs(&self) -> Vec<SpringState> {
        intersperse(self.broken_cardinalities.iter().map(|n: &usize| vec![SpringState::Broken; *n]), vec![SpringState::Operational]).flatten().collect()
    }

    pub fn complete_entry_permutations(&self) -> impl Iterator<Item=Self> + '_ {
        vec![].into_iter()
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


pub fn solve(lines: Vec<String>) {
    if let Ok(record) = ConditionRecord::try_from(lines) {
        let total_permutation_count: usize = record.entries.into_iter().map(|e| e.complete_entry_permutations().count()).sum();
        println!("{}", total_permutation_count);
    }
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
        assert_eq!(entry.broken_cardinalities, vec![1, 1, 3])
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
        assert!(ConditionRecordEntry::try_from(".#.###.#.###### 1,3,1,6".to_string()).unwrap().is_acceptable_entry());
        assert!(ConditionRecordEntry::try_from(".#.###.#.######. 1,3,1,6".to_string()).unwrap().is_acceptable_entry());
        assert!(ConditionRecordEntry::try_from("#.###.#.###### 1,3,1,6".to_string()).unwrap().is_acceptable_entry());
        assert!(ConditionRecordEntry::try_from(".#...#....###. 1,1,3".to_string()).unwrap().is_acceptable_entry());
        assert!(ConditionRecordEntry::try_from(".#.#?#.#.?????? 1,3,1,6".to_string()).unwrap().is_acceptable_entry());
        assert!(!ConditionRecordEntry::try_from(".##.##.#.###### 1,3,1,6".to_string()).unwrap().is_acceptable_entry());
    }

    #[test]
    fn known_accepted_springs() {
        assert_eq!(
            ConditionRecordEntry::try_from("???.### 1,1,3".to_string()).unwrap().known_accepted_springs(),
            vec![
                SpringState::Broken, SpringState::Operational, SpringState::Broken,
                SpringState::Operational,
                SpringState::Broken, SpringState::Broken, SpringState::Broken
            ]
        );
    }

    // #[test]
    // fn one_permutation_entry() {
    //     let perms: Vec<ConditionRecordEntry> = ConditionRecordEntry::try_from("???.### 1,1,3".to_string()).unwrap().complete_entry_permutations().collect();
    //     assert_eq!(perms.len(), 1);
    //     // let perm: String = perms.get(0).unwrap().springs.into_iter().map(|ss| ss.try_into().ok().unwrap()).collect::<String>();
    //     // assert_eq!(perm, "#.#.###".to_string());
    // }

    // #[test]
    // fn multiple_permutation_entry() {
    //     let perms: Vec<ConditionRecordEntry> = ConditionRecordEntry::try_from("?###???????? 3,2,1".to_string()).unwrap().complete_entry_permutations().collect();
    //     assert_eq!(perms.len(), 10);
    //     // let perm: String = perms.get(0).unwrap().springs.into_iter().map(|ss| ss.try_into().ok().unwrap()).collect::<String>();
    //     // assert_eq!(perm, "#.#.###".to_string());
    // }
}
