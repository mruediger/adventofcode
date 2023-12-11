#[derive(PartialEq, Debug)]
pub struct Array2D {
    content: Vec<Vec<char>>,
}

impl FromIterator<Vec<char>> for Array2D {
    fn from_iter<I: IntoIterator<Item = Vec<char>>>(iter: I) -> Self {
        let mut content: Vec<Vec<char>> = Vec::new();
        for i in iter {
            content.push(i)
        }

        Array2D { content }
    }
}

impl std::fmt::Display for Array2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in &self.content {
            for c in line {
                write!(f, "{}", c)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Array2D {
    fn rows(&self) -> Vec<Vec<char>> {
        let mut result = Vec::new();
        for row in &self.content {
            result.push(row.clone());
        }
        result
    }

    fn collumns(&self) -> Vec<Vec<char>> {
        if self.content.is_empty() {
            return Vec::new();
        }
        let mut result = vec![Vec::new(); self.content[0].len()];

        for row in &self.content {
            for (i, c) in row.iter().enumerate() {
                result[i].push(*c);
            }
        }

        result
    }

    fn add_row(&mut self, i: usize, row: Vec<char>) {
        self.content.insert(i, row);
    }

    fn add_collumn(&mut self, i: usize, collumn: Vec<char>) {
        let mut collumn = collumn.clone();

        for row in &mut self.content {
            row.insert(i, collumn.pop().unwrap())
        }
    }
}

type Point = (usize, usize);
pub type Galaxy = Array2D;

impl Galaxy {
    pub fn expand(&mut self, times: usize) {
        let mut offset = 0;
        for (i, row) in self.rows().iter().enumerate() {
            if !row.contains(&'#') {
                for _ in 0..times {
                    self.add_row(i + offset, row.clone());
                    offset += 1;
                }
            }
        }
        offset = 0;
        for (i, collumn) in self.collumns().iter().enumerate() {
            if !collumn.contains(&'#') {
                for _ in 0..times {
                    self.add_collumn(i + offset, collumn.clone());
                    offset += 1;
                }
            }
        }
    }

    fn galaxies(&self) -> Vec<Point> {
        let mut galaxies = Vec::new();

        for (x, row) in self.content.iter().enumerate() {
            for (y, c) in row.iter().enumerate() {
                if *c != '.' {
                    galaxies.push((x, y));
                }
            }
        }
        galaxies
    }

    pub fn pairs(&self) -> Vec<(Point, Point)> {
        let mut pairs = Vec::new();

        for (i, point) in self.galaxies().iter().enumerate() {
            pairs.append(&mut self.pairs_from(i, point));
        }

        pairs
    }

    pub fn pairs_from(&self, i_1: usize, point_1: &Point) -> Vec<(Point, Point)> {
        let mut pairs: Vec<(Point, Point)> = Vec::new();

        for (i, point) in self.galaxies().iter().enumerate() {
            if i <= i_1 {
                continue;
            }
            pairs.push((*point_1, *point));
        }
        pairs
    }

    pub fn distance(&self, a: &Point, b: &Point) -> u32 {
        (a.0 as i32).abs_diff(b.0 as i32) + (a.1 as i32).abs_diff(b.1 as i32)
    }
}

pub fn parse(input: &str) -> Galaxy {
    input.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {

    use indoc::indoc;

    const INPUT: &str = indoc! {"...#......
                                 .......#..
                                 #.........
                                 ..........
                                 ......#...
                                 .#........
                                 .........#
                                 ..........
                                 .......#..
                                 #...#....."};

    const EXPANDED_INPUT: &str = indoc! {"....#........
                                          .........#...
                                          #............
                                          .............
                                          .............
                                          ........#....
                                          .#...........
                                          ............#
                                          .............
                                          .............
                                          .........#...
                                          #....#......."};

    #[test]
    fn expand() {
        let mut galaxy = super::parse(INPUT);
        galaxy.expand(1);
        assert_eq!(galaxy, super::parse(EXPANDED_INPUT));
    }

    #[test]
    fn pairs() {
        let galaxy = super::parse(EXPANDED_INPUT);
        let pairs = galaxy.pairs();
        assert_eq!(pairs.len(), 36);
    }

    #[test]
    fn distance() {
        let galaxy = super::parse(EXPANDED_INPUT);
        assert_eq!(galaxy.distance(&(0, 4), &(10, 9)), 15);
        assert_eq!(galaxy.distance(&(2, 0), &(7, 12)), 17);
        assert_eq!(galaxy.distance(&(11, 0), &(11, 5)), 5);
        assert_eq!(galaxy.distance(&(10, 9), &(0, 4)), 15);
        assert_eq!(galaxy.distance(&(7, 12), &(2, 0)), 17);
        assert_eq!(galaxy.distance(&(11, 5), &(11, 0)), 5);
    }

    #[test]
    fn all_distances() {
        let galaxy = super::parse(EXPANDED_INPUT);
        let sum: u32 = galaxy
            .pairs()
            .iter()
            .map(|(a, b)| galaxy.distance(a, b))
            .sum();
        assert_eq!(sum, 374);
    }
}
