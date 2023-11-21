use itertools::Itertools;

fn group_calories(lines: Vec<String>) -> Vec<Vec<u32>> {
    let mut calorie_groups: Vec<Vec<u32>> = Vec::new();
    for (key, group) in &lines.into_iter().group_by(|line| !line.is_empty()) {
        if key {
            calorie_groups.push(group.map(|calorie| calorie.parse::<u32>().unwrap()).collect::<Vec<u32>>());
        }
    }
    calorie_groups
}

fn sum_calories(elf_groups: Vec<Vec<u32>>) -> Vec<u32> {
    elf_groups.into_iter().map(|group : Vec<u32>| group.into_iter().sum()).collect()
}

fn largest_n_calories(total_calories: Vec<u32>, n: usize) -> Vec<u32> {
    total_calories.into_iter().sorted_by(|a, b| Ord::cmp(&b, &a)).take(n).collect()
}

pub fn solve(lines: Vec<String>) {
    let calorie_sum: Vec<u32> = sum_calories(group_calories(lines));
    let calorie_maximal = | n: usize | largest_n_calories(calorie_sum.clone(), n).into_iter().sum::<u32>();
    println!("{}", calorie_maximal(1));
    println!("{}", calorie_maximal(3));
}

#[cfg(test)]
mod calorie_count {
    use crate::year_2022::day_1::calorie_count::largest_n_calories;

    use super::{group_calories, sum_calories};

    #[test]
    fn multi_group_calories() {
        let input = vec!["1000", "2000", "3000", "", "4000", "", "5000", "6000"].into_iter().map(|str| str.to_owned()).collect();
        assert_eq!(group_calories(input), vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000]]) 
    }

    #[test]
    fn no_group_calories() {
        let input: Vec<String> = vec![];
        assert_eq!(group_calories(input), vec![] as Vec<Vec<u32>>); 
    }

    #[test]
    fn sum_multiple_calorie_groups() {
        let input = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000]];
        assert_eq!(sum_calories(input), vec![6000, 4000, 11000]);
    }

    #[test]
    fn sum_no_calorie_groups() {
        let input: Vec<Vec<u32>> = vec![];
        assert_eq!(sum_calories(input), vec![]);
    }

    #[test]
    fn largest_two_calories_from_group() {
        assert_eq!(largest_n_calories(vec![6000, 7000, 3000, 10000], 2), vec![10000, 7000]);
    }
    
    #[test]
    fn largest_calorie_from_group() {
        assert_eq!(largest_n_calories(vec![6000, 3000, 10000], 1), vec![10000]);
    }

    #[test]
    fn largest_calorie_from_singleton() {
        assert_eq!(largest_n_calories(vec![6000], 1), vec![6000]);
    }

    #[test]
    fn largest_calorie_from_none() {
        assert_eq!(largest_n_calories(vec![], 1), vec![]);
    }

    #[test]
    fn largest_3_calories_from_none() {
        assert_eq!(largest_n_calories(vec![], 1), vec![]);
    }

}