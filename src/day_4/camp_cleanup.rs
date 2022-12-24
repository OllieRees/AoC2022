use itertools::Itertools;

use crate::trim_input;

type ElfPair = (u32, u32);

fn has_superset(pair_a: ElfPair, pair_b: ElfPair) -> bool {
    let a_superset = pair_a.1 >= pair_b.1 && pair_a.0 <= pair_b.0;
    let b_superset = pair_b.1 >= pair_a.1 && pair_b.0 <= pair_a.0;
    a_superset || b_superset
}

fn has_overlap(pair_a: ElfPair, pair_b: ElfPair) -> bool {
    !(pair_a.0 > pair_b.1 || pair_a.1 < pair_b.0)
}

fn parse_elf_set(range_str: String) -> ElfPair {
    range_str
        .split("-")
        .map(|rm| rm.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn parse_elf_pair(line: String) -> (ElfPair, ElfPair) {
    line.split(",")
        .map(|elf| parse_elf_set(elf.to_owned()))
        .collect_tuple()
        .unwrap()
}

pub fn main(lines: Vec<String>) {
    let lines = trim_input(lines);
    let mut superset_cntr = 0;
    let mut overlap_cntr = 0;
    for line in lines {
        let elf_pair = parse_elf_pair(line);
        if has_superset(elf_pair.0, elf_pair.1) {
            overlap_cntr += 1;
            superset_cntr += 1;
        } else if has_overlap(elf_pair.0, elf_pair.1) {
            overlap_cntr += 1;
        }
    }
    println!("Total Supersets: {superset_cntr}");
    println!("Total overlaps: {overlap_cntr}");
}
