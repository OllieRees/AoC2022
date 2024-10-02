use std::collections::HashSet;


struct Path {
    start_point: (usize, usize),
    end_point: (usize, usize)
}

impl Path {
    pub fn shortest_path_length(&self) -> usize {
        usize::abs_diff(self.end_point.0, self.start_point.0) + usize::abs_diff(self.end_point.1, self.start_point.1)
    }
}


#[derive(Debug, Hash, PartialEq, Eq)]
struct Galaxy {
    coord: (usize, usize)
}

struct Image {
    size: (usize, usize),
    galaxies: HashSet<Galaxy>
}

impl Image {
    fn expandable_row_indices(&self) -> impl Iterator<Item=usize> + '_ {
        let range: HashSet<usize> = HashSet::from_iter(0..self.size.0);
        let galaxy_rows: HashSet<usize> = self.galaxies.iter().map(|galaxies| galaxies.coord.0).collect();
        (&range - &galaxy_rows).into_iter()
    }
    fn expandable_column_indices(&self) -> impl Iterator<Item=usize> + '_ {
        let range: HashSet<usize> = HashSet::from_iter(0..self.size.1);
        let galaxy_rows: HashSet<usize> = self.galaxies.iter().map(|galaxies| galaxies.coord.1).collect();
        (&range - &galaxy_rows).into_iter()
    }
    pub fn expandable_position_of_galaxy(&self, galaxy: &Galaxy) -> Galaxy {
        let y_delta: usize = self.expandable_row_indices().filter(|x| *x < galaxy.coord.0).count();
        let x_delta: usize = self.expandable_column_indices().filter(|x| *x < galaxy.coord.1).count();
        Galaxy { coord: (galaxy.coord.0 + y_delta, galaxy.coord.1 + x_delta) }
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
}


#[cfg(test)]
mod test_cosmic_expansion {
    use itertools::Itertools;

    use super::{Galaxy, HashSet, Image};

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
        assert_eq!(image.expandable_position_of_galaxy(&Galaxy {coord: (0, 3)}), Galaxy{coord: (0, 4)});
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
        assert_eq!(image.expandable_position_of_galaxy(&Galaxy {coord: (5, 1)}), Galaxy{coord: (6, 1)});
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
        assert_eq!(image.expandable_position_of_galaxy(&Galaxy {coord: (8, 7)}), Galaxy{coord: (10, 9)});
    }
}
