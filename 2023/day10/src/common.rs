pub type Point = (usize, usize);

#[derive(Clone, PartialEq, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Sketch {
    chars: Vec<Vec<char>>,
    current_pos: Point,
}

impl Sketch {
    fn get(&self, pos: Point) -> Option<char> {
        if let Some(a) = self.chars.get(pos.0) {
            a.get(pos.1).copied()
        } else {
            println!("{:?}", pos);
            None
        }
    }

    pub fn get_loop(&self) -> Vec<Point> {
        let mut direction = Dir::Up;
        let start_pos = self.current_pos;
        let mut current_pos = start_pos;
        let mut points = Vec::new();

        while current_pos != start_pos || points.is_empty() {
            let current_char = self.get(current_pos);
            let dirs = match current_char {
                Some('|') => vec![Dir::Up, Dir::Down],
                Some('-') => vec![Dir::Left, Dir::Right],
                Some('L') => vec![Dir::Up, Dir::Right],
                Some('J') => vec![Dir::Up, Dir::Left],
                Some('7') => vec![Dir::Down, Dir::Left],
                Some('F') => vec![Dir::Down, Dir::Right],
                Some('S') => vec![Dir::Up, Dir::Down],
                _ => panic!("unexpected pipe {:?}", current_char),
            };

            direction = match direction {
                Dir::Up => dirs.iter().find(|d| d != &&Dir::Down).unwrap().clone(),
                Dir::Down => dirs.iter().find(|d| d != &&Dir::Up).unwrap().clone(),
                Dir::Left => dirs.iter().find(|d| d != &&Dir::Right).unwrap().clone(),
                Dir::Right => dirs.iter().find(|d| d != &&Dir::Left).unwrap().clone(),
            };

            current_pos = step(current_pos, direction.clone());
            points.push(current_pos);
        }
        points
    }
}

pub fn parse(input: &str) -> Option<Sketch> {
    let chars: Vec<Vec<char>> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    for (x, line) in chars.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == 'S' {
                let current_pos = (x, y);
                return Some(Sketch { chars, current_pos });
            }
        }
    }

    None
}

fn step(current_pos: Point, direction: Dir) -> Point {
    match direction {
        Dir::Up => (current_pos.0 - 1, current_pos.1),
        Dir::Down => (current_pos.0 + 1, current_pos.1),
        Dir::Left => (current_pos.0, current_pos.1 - 1),
        Dir::Right => (current_pos.0, current_pos.1 + 1),
    }
}

#[cfg(test)]
mod test {

    const INPUT: &str = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn parse_input() {
        let sketch = super::parse(INPUT).unwrap();
        assert_eq!(sketch.get_loop().len(), 16);
    }

    #[test]
    fn get() {
        let sketch = super::parse(INPUT).unwrap();
        assert_eq!(sketch.get((0, 0)), Some('7'));
    }

    #[test]
    fn step() {
        assert_eq!(super::step((5, 1), super::Dir::Up), (4, 1));
    }
}
