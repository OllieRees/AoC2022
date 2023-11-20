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
