use itertools::Itertools;

fn sum_calories(lines: Vec<String>) -> Vec<u32> {
    let grouped_elves: Vec<&[String]> = lines.split(|calorie: &String| calorie.eq("")).collect();

    grouped_elves
        .iter()
        .map(|calories: &&[String]| {
            calories
                .into_iter()
                .map(|calorie| {
                    if let Ok(x) = calorie.parse::<u32>() {
                        return x;
                    }
                    0
                })
                .sum::<u32>()
        })
        .collect()
}

pub fn main(lines: Vec<String>) {
    let calorie_sum: Vec<u32> = sum_calories(lines);
    println!("{}", &calorie_sum.iter().max().unwrap());
    println!(
        "{}",
        &calorie_sum.into_iter().sorted().rev().take(3).sum::<u32>()
    );
}
