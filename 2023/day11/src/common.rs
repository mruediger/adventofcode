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

#[allow(dead_code)]
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
            row.append(&mut vec!['.'; 100000]);
            row.insert(i, collumn.pop().unwrap())
        }
    }

    fn add_collumns(&mut self, i: usize, count: usize, collumn: Vec<char>) {
        let mut collumn = collumn.clone();

        for row in &mut self.content {
            let mut tail = row.split_off(i);
            row.append(&mut vec![collumn.pop().unwrap(); count]);
            row.append(&mut tail);
        }
    }
}

type Point = (usize, usize);

#[derive(Debug, PartialEq)]
pub struct Universe {
    galaxies: Vec<Point>,
}

impl Universe {
    pub fn expand(&mut self, times: usize) {
        let (height, width) = (
            &self.galaxies.iter().max_by_key(|(x, _)| x).unwrap().0 + 1,
            &self.galaxies.iter().max_by_key(|(_, y)| y).unwrap().1 + 1,
        );

        let mut rows = vec![false; height];
        let mut cols = vec![false; width];

        for galaxy in &self.galaxies {
            rows[galaxy.0] = true;
            cols[galaxy.1] = true;
        }

        for (i, (x, _)) in rows.iter().enumerate().filter(|(_, &row)| !row).enumerate() {
            for galaxy in &mut self.galaxies {
                if galaxy.0 > x + (i * times) {
                    galaxy.0 += times;
                }
            }
        }

        for (i, (y, _)) in cols.iter().enumerate().filter(|(_, &col)| !col).enumerate() {
            for galaxy in &mut self.galaxies {
                if galaxy.1 > y + (i * times) {
                    galaxy.1 += times;
                }
            }
        }
    }

    pub fn pairs(&self) -> Vec<(Point, Point)> {
        let mut pairs = Vec::new();

        for (i, point) in self.galaxies.iter().enumerate() {
            pairs.append(&mut self.pairs_from(i, point));
        }

        pairs
    }

    pub fn pairs_from(&self, i_1: usize, point_1: &Point) -> Vec<(Point, Point)> {
        let mut pairs: Vec<(Point, Point)> = Vec::new();

        for (i, point) in self.galaxies.iter().enumerate() {
            if i <= i_1 {
                continue;
            }
            pairs.push((*point_1, *point));
        }
        pairs
    }

    pub fn distance(&self, a: &Point, b: &Point) -> usize {
        (a.0).abs_diff(b.0) + (a.1).abs_diff(b.1)
    }
}

pub fn parse(input: &str) -> Universe {
    let mut galaxies = Vec::new();

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
            }
        }
    }

    Universe { galaxies }
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

    const EXPANDED_INPUT2: &str = indoc! {".....#..........
                                           ...........#....
                                           #...............
                                           ................
                                           ................
                                           ................
                                           ..........#.....
                                           .#..............
                                           ...............#
                                           ................
                                           ................
                                           ................
                                           ...........#....
                                           #.....#........."};

    #[test]
    fn expand() {
        let mut galaxy = super::parse(INPUT);
        galaxy.expand(1);
        assert_eq!(galaxy, super::parse(EXPANDED_INPUT));
    }

    #[test]
    fn expand2() {
        let mut galaxy = super::parse(INPUT);
        galaxy.expand(2);
        assert_eq!(galaxy, super::parse(EXPANDED_INPUT2));
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
        let sum: usize = galaxy
            .pairs()
            .iter()
            .map(|(a, b)| galaxy.distance(a, b))
            .sum();
        assert_eq!(sum, 374);
    }

    #[test]
    fn all_distances_10() {
        let mut galaxy = super::parse(INPUT);
        galaxy.expand(9);
        let sum: usize = galaxy
            .pairs()
            .iter()
            .map(|(a, b)| galaxy.distance(a, b))
            .sum();
        assert_eq!(sum, 1030);
    }

    #[test]
    fn all_distances_100() {
        let mut galaxy = super::parse(INPUT);
        galaxy.expand(99);
        let sum: usize = galaxy
            .pairs()
            .iter()
            .map(|(a, b)| galaxy.distance(a, b))
            .sum();
        assert_eq!(sum, 8410);
    }
}
