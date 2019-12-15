extern crate itertools;
extern crate num;

use itertools::Itertools;
use num::Integer;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Angle {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, Ord, Eq)]
struct Astorid {
    x: usize,
    y: usize,
    sees: usize,
}

impl Astorid {
    fn mark_intersections(&self, astorids: Vec<Astorid>) -> Astorid {
        let mut last = Angle { x: 0, y: 0 };
        let sees = astorids
            .into_iter()
            .filter(|a| self != a)
            .map(|b| (self.angle(&b), b))
            .sorted_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .fold(0, |sum, (i, _)| {
                if i != last {
                    last = i.clone();
                    sum + 1
                } else {
                    sum
                }
            });

        Astorid {
            x: self.x,
            y: self.y,
            sees: sees,
        }
    }

    fn angle(&self, astorid: &Astorid) -> Angle {
        let x = astorid.x as isize - self.x as isize;
        let y = astorid.y as isize - self.y as isize;

        let gcd = x.gcd(&y);

        if gcd == 0 {
            Angle { x: 0, y: 0 }
        } else {
            Angle {
                x: x / gcd,
                y: y / gcd,
            }
        }
    }
}

impl PartialEq for Astorid {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl PartialOrd for Astorid {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let ordering = self.x.partial_cmp(&rhs.x);

        if ordering == Some(Ordering::Equal) {
            self.y.partial_cmp(&rhs.y)
        } else {
            ordering
        }
    }
}

#[test]
fn test_1() {
    let astorids: Vec<Astorid> = ".#..#\n.....\n#####\n....#\n...##"
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|&(_, _, c)| c == '#')
        .map(|(x, y, _)| Astorid {
            x: x,
            y: y,
            sees: 0,
        })
        .collect();

    let max = astorids
        .iter()
        .map(|a| a.mark_intersections(astorids.clone()))
        .max_by_key(|a| a.sees);

    assert_eq!(
        max,
        Some(Astorid {
            x: 3,
            y: 4,
            sees: 0
        })
    );
}

fn main() {
    let astorids: Vec<Astorid> = std::fs::read_to_string("../input.txt")
        .expect("error reading input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|&(_, _, c)| c == '#')
        .map(|(x, y, _)| Astorid {
            x: x,
            y: y,
            sees: 0,
        })
        .collect();

    let max = astorids
        .iter()
        .map(|a| a.mark_intersections(astorids.clone()))
        .max_by_key(|a| a.sees);

    println!("{:?}", max)
}
