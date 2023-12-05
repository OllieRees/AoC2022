use super::parse_input::parse;

type Range = (u64, u64, u64);

fn is_in_range(x: u64, range: Range) -> bool {
    x >= range.0 && x < range.0 + range.2
}

fn seed_location(seed: u64, mappings: &Vec<Vec<Range>>) -> u64 {
    let get_next_value = |seed: u64, ranges: &Vec<Range>| -> u64 {
        match ranges.iter().find(|range: &&(u64, u64, u64)| is_in_range(seed, **range)) {
            Some(range) => range.1 + (seed - range.0),
            None => seed,
        }
    };
    mappings.into_iter().fold(seed, get_next_value)
}

fn reverse_mapping(mappings: Vec<Vec<Range>>) -> Vec<Vec<Range>> {
    mappings.into_iter().map(|v: Vec<(u64, u64, u64)>| v.into_iter().map(|range: (u64, u64, u64)| (range.1, range.0, range.2)).collect()).rev().collect()
}

pub fn solve(lines: Vec<String>) {
    if let Ok((seeds, mappings)) = parse(lines) {
        println!("Minimum Location {}", seeds.iter().map(|seed| seed_location(*seed, &mappings)).min().unwrap());
        let seeds: Vec<(u64, u64)> = seeds.chunks(2).map(|x| (x[0], x[1])).collect();
        let mappings: Vec<Vec<Range>> = reverse_mapping(mappings);
        println!("Minimum Location whe Seeds are Ranges {}", (1..).find(|location: &u64| {
            let seed: u64 = seed_location(*location, &mappings);
            seeds.iter().any(|(s, n)| seed >= *s && seed < s + n)
        }).unwrap());
    }
}

#[cfg(test)]
pub mod planting_seeds {
    use crate::year_2023::day_5::planting_seeds::*;

    #[test]
    fn test_seed_location() {
        let mappings: Vec<Vec<Range>> =      
            vec![
                vec![(98, 50, 2), (50, 52, 48)],
                vec![(15, 0, 37), (52, 37, 2), (0, 39, 15)],
                vec![(53, 49, 8), (11, 0, 42), (0, 42, 7), (7, 57, 4)],
                vec![(18, 88, 7), (25, 18, 70)],
                vec![(77, 45, 23), (45, 81, 19), (64, 68, 13)],
                vec![(69, 0, 1), (0, 1, 69)],
                vec![(56, 60, 37), (93, 56, 4)]
            ];
        assert_eq!(seed_location(79, &mappings), 82);
        assert_eq!(seed_location(14, &mappings), 43);
        assert_eq!(seed_location(55, &mappings), 86);
        assert_eq!(seed_location(13, &mappings), 35);
    }

    #[test]
    fn test_inverse_seed_location() {
        let mappings: Vec<Vec<Range>> =      
            vec![
                vec![(98, 50, 2), (50, 52, 48)],
                vec![(15, 0, 37), (52, 37, 2), (0, 39, 15)],
                vec![(53, 49, 8), (11, 0, 42), (0, 42, 7), (7, 57, 4)],
                vec![(18, 88, 7), (25, 18, 70)],
                vec![(77, 45, 23), (45, 81, 19), (64, 68, 13)],
                vec![(69, 0, 1), (0, 1, 69)],
                vec![(56, 60, 37), (93, 56, 4)]
            ];
        let mappings = reverse_mapping(mappings);
        assert_eq!(seed_location(82, &mappings), 79);
        assert_eq!(seed_location(43, &mappings), 14);
        assert_eq!(seed_location(86, &mappings), 55);
        assert_eq!(seed_location(35, &mappings), 13);
    }

    #[test]
    fn test_pair_up_seeds() {
        let input: Vec<u64> = vec![79, 14, 55, 13];
        let pairs: Vec<(u64, u64)> = input.chunks(2).map(|x| (x[0], x[1])).collect();
        assert_eq!(pairs, vec![(79, 14), (55, 13)]);
    }
}