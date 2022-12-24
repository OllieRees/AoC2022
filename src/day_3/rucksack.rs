use std::collections::HashSet;

use itertools::Itertools;

use crate::trim_input;

type Item = char;
type Compartment = Vec<Item>;
type Rucksack = (Compartment, Compartment);
type ElfGroup = (Rucksack, Rucksack, Rucksack);

fn item_priority(item: &Item) -> u8 {
    let item = *item;
    if item.is_lowercase() {
        return (item as u8) - 96;
    } else if item.is_uppercase() {
        return (item as u8) - 38;
    }
    0
}

fn sum_item_priorities(items: Vec<Item>) -> u32 {
    items
        .iter()
        .map(|item: &Item| item_priority(item) as u32)
        .sum()
}

fn new_compartment(items: String) -> Compartment {
    items.chars().collect()
}

fn common_items(compartment_a: &Compartment, compartment_b: &Compartment) -> Compartment {
    let compartment_a: HashSet<char> = HashSet::from_iter(compartment_a.into_iter().map(|i| *i));
    let compartment_b: HashSet<char> = HashSet::from_iter(compartment_b.into_iter().map(|i| *i));
    compartment_a
        .intersection(&compartment_b)
        .map(|i| *i)
        .collect()
}

fn common_item(compartment_a: &Compartment, compartment_b: &Compartment) -> Item {
    *common_items(compartment_a, compartment_b).get(0).unwrap()
}

fn merge_rucksack(rucksack: Rucksack) -> Compartment {
    rucksack.0.into_iter().chain(rucksack.1).collect()
}

fn get_elfgroup_badge(group: ElfGroup) -> Item {
    let (group_a, group_b, group_c) = group;
    let init_set = common_items(&merge_rucksack(group_a), &merge_rucksack(group_b));
    common_item(&init_set, &merge_rucksack(group_c))
}

fn collect_elf_group(rucksacks: Vec<Rucksack>) -> Vec<ElfGroup> {
    let mut group = Vec::new();
    for (_, enum_grp) in &rucksacks.into_iter().enumerate().group_by(|(i, _)| i / 3) {
        let enum_grp: ElfGroup = enum_grp.map(|(_, e)| e).collect_tuple().unwrap();
        group.push(enum_grp);
    }
    group
}

fn parse_rucksack(line: String) -> Rucksack {
    let mid = line.len() / 2;
    let (comp_1, comp_2) = line.split_at(mid);

    (
        new_compartment(comp_1.to_owned()),
        new_compartment(comp_2.to_owned()),
    )
}

pub fn main(lines: Vec<String>) {
    let lines = trim_input(lines);
    let rucksacks: Vec<Rucksack> = lines
        .iter()
        .map(|line: &String| parse_rucksack(line.clone()))
        .collect();
    let common_items: Vec<Item> = rucksacks
        .iter()
        .map(|rucksack: &Rucksack| common_item(&rucksack.0, &rucksack.1))
        .collect();
    let total_priority: u32 = sum_item_priorities(common_items);
    println!("Total priority sum is {}", total_priority);

    let elfgroup_badges: Vec<Item> = collect_elf_group(rucksacks)
        .into_iter()
        .map(|group| get_elfgroup_badge(group))
        .collect();
    let total_badge_priorities: u32 = sum_item_priorities(elfgroup_badges);
    println!("Elf Groups: {}", total_badge_priorities);
}
