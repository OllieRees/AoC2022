use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Galaxy {
    coord: (usize, usize)
}

struct Image {
    size: (usize, usize),
    galaxies: HashSet<Galaxy>
}

impl Image {
    pub fn expandable_rows(&self) -> impl Iterator<Item=usize> + '_ {
        let range: HashSet<usize> = HashSet::from_iter(0..self.size.0);
        let galaxy_rows: HashSet<usize> = self.galaxies.iter().map(|galaxies| galaxies.coord.0).collect();
        (&range - &galaxy_rows).into_iter()
    }
    pub fn expandable_columns(&self) -> impl Iterator<Item=usize> + '_ {
        let range: HashSet<usize> = HashSet::from_iter(0..self.size.1);
        let galaxy_rows: HashSet<usize> = self.galaxies.iter().map(|galaxies| galaxies.coord.1).collect();
        (&range - &galaxy_rows).into_iter()
    }
}

impl From<Vec<String>> for Image {
    fn from(value: Vec<String>) -> Self {
        let size = (value.len(), value.get(0).unwrap_or(&"".to_string()).len());
        let galaxies: HashSet<Galaxy> = value.into_iter().enumerate().map(|(row_n, row)| {
                row.match_indices('#').map(|(col_n, _)| Galaxy {coord: (row_n, col_n)}).collect::<HashSet<Galaxy>>()
            }).flatten().collect();
        Image {size, galaxies}
    }
}

pub fn solve(lines: Vec<String>) {

}

#[cfg(test)]
mod test_cosmic_expansion {
    use itertools::Itertools;

    use super::{Galaxy, Image, HashSet};

    #[test]
    fn capture_image() {
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

    #[test]
    fn capture_empty_image() {
        let image: Image = Image::from(vec![]);
        assert_eq!(image.size, (0, 0));
        assert_eq!(image.galaxies, HashSet::new());
    }

    #[test]
    fn expandable_rows() {
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
        assert_eq!(image.expandable_rows().sorted().collect::<Vec<usize>>(), vec![3,7]);
    }

    #[test]
    fn expandable_cols() {
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
        assert_eq!(image.expandable_columns().sorted().collect::<Vec<usize>>(), vec![2,5,8]);
    }
}
