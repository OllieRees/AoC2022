use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Galaxy {
    coord: (usize, usize)
}

struct Image {
    galaxies: HashSet<Galaxy>
}

impl From<Vec<String>> for Image {
    fn from(value: Vec<String>) -> Self {
        let galaxies: HashSet<Galaxy> = value.into_iter().enumerate().map(|(row_n, row)| {
                row.match_indices('#').map(|(col_n, _)| Galaxy {coord: (row_n, col_n)}).collect::<HashSet<Galaxy>>()
            }).flatten().collect();
        Image {galaxies}
    }
}

pub fn solve(lines: Vec<String>) {

}

#[cfg(test)]
mod test_cosmic_expansion {
    use super::{Galaxy, Image};

    #[test]
    fn test_capture_image() {
        let image: Image = Image::from(vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ]);
        assert_eq!(image.galaxies.len(), 9);
        assert!(image.galaxies.contains(&Galaxy{coord: (0, 3)}));
        assert!(image.galaxies.contains(&Galaxy{coord: (1, 7)}));
        assert!(image.galaxies.contains(&Galaxy{coord: (2, 0)}));
        assert!(image.galaxies.contains(&Galaxy{coord: (4, 6)}));
        assert!(image.galaxies.contains(&Galaxy{coord: (5, 1)}));
        assert!(image.galaxies.contains(&Galaxy{coord: (6, 9)}));
        assert!(image.galaxies.contains(&Galaxy{coord: (8, 7)}));
        assert!(image.galaxies.contains(&Galaxy{coord: (9, 0)}));
        assert!(image.galaxies.contains(&Galaxy{coord: (9, 4)}));
    }
}
