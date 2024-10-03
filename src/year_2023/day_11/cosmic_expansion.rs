use std::collections::HashSet;

use itertools::Itertools;


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Galaxy {
    coord: (usize, usize)
}

impl Galaxy {
    pub fn shortest_path_to_other_galaxy(&self, other: &Galaxy) -> usize {
        usize::abs_diff(other.coord.0, self.coord.0) + usize::abs_diff(other.coord.1, self.coord.1)
    }
}

struct Image {
    size: (usize, usize),
    galaxies: HashSet<Galaxy>
}

impl Image {
    fn expandable_row_indices(&self) -> impl Iterator<Item=usize> + '_ {
        (&HashSet::from_iter(0..self.size.0) - &self.galaxies.iter().map(|galaxies| galaxies.coord.0).collect::<HashSet<usize>>()).into_iter()
    }

    fn expandable_column_indices(&self) -> impl Iterator<Item=usize> + '_ {
        (&HashSet::from_iter(0..self.size.1) - &self.galaxies.iter().map(|galaxies| galaxies.coord.1).collect::<HashSet<usize>>()).into_iter()
    }

    fn galaxy_in_expanded_image(&self, galaxy: &Galaxy, factor: usize) -> Galaxy {
        let y_delta: usize = (factor - 1) * self.expandable_row_indices().filter(|x| *x < galaxy.coord.0).count();
        let x_delta: usize = (factor - 1) * self.expandable_column_indices().filter(|x| *x < galaxy.coord.1).count();
        Galaxy { coord: (galaxy.coord.0 + y_delta, galaxy.coord.1 + x_delta) }
    }

    pub fn expand_image(&self, factor: usize) -> Self {
        Image {
            size: (self.size.0 + ((factor - 1) * self.expandable_row_indices().count()), self.size.1 + ((factor - 1) * self.expandable_column_indices().count())),
            galaxies: self.galaxies.iter().map(|g| self.galaxy_in_expanded_image(g, factor)).collect()
        }
    }

    pub fn galaxy_pairs(&self) -> impl Iterator<Item=(&Galaxy, &Galaxy)> + '_ {
        self.galaxies.iter().combinations(2).map(|v| (v[0], v[1]))
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
    let image: Image = Image::from(lines);
    let shortest_paths: usize = image.expand_image(2).galaxy_pairs().map(|(x, y)| x.shortest_path_to_other_galaxy(y)).sum();
    println!("Expansion factor of 2: {}", shortest_paths);
    let shortest_paths: usize = image.expand_image(1000000).galaxy_pairs().map(|(x, y)| x.shortest_path_to_other_galaxy(y)).sum();
    println!("Expansion factor of 1,000,000: {}", shortest_paths);
}


#[cfg(test)]
mod test_cosmic_expansion {
    use itertools::Itertools;

    use super::{Galaxy, HashSet, Image};

    #[test]
    fn shortest_path_between_galaxies() {
        assert_eq!(Galaxy {coord: (6, 1)}.shortest_path_to_other_galaxy(&Galaxy {coord: (11, 5)}), 9);
    }

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
        assert_eq!(image.expandable_row_indices().sorted().collect::<Vec<usize>>(), vec![3,7]);
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
        assert_eq!(image.expandable_column_indices().sorted().collect::<Vec<usize>>(), vec![2,5,8]);
    }

    #[test]
    fn galaxy_expands_by_one_column() {
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
        assert_eq!(image.galaxy_in_expanded_image(&Galaxy {coord: (0, 3)}, 2), Galaxy{coord: (0, 4)});
    }

    #[test]
    fn galaxy_expands_by_one_row() {
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
        assert_eq!(image.galaxy_in_expanded_image(&Galaxy {coord: (5, 1)}, 2), Galaxy{coord: (6, 1)});
    }

    #[test]
    fn galaxy_expands_by_rows_and_columns() {
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
        assert_eq!(image.galaxy_in_expanded_image(&Galaxy {coord: (8, 7)}, 2), Galaxy{coord: (10, 9)});
    }

    #[test]
    fn galaxy_pairs() {
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
        let galaxy_pairs: Vec<(&Galaxy, &Galaxy)> = image.galaxy_pairs().collect();
        assert_eq!(galaxy_pairs.len(), 36);
    }
}
