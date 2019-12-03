#[macro_use]
extern crate itertools;

use std::fs;

const INPUT: &str = "../input.txt";

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    pub fn normalized(&self) -> Line {
        if self.is_horizontal() {
            if self.a.x < self.b.x {
                Line {
                    a: self.a,
                    b: self.b,
                }
            } else {
                Line {
                    a: self.b,
                    b: self.a,
                }
            }
        } else {
            if self.a.y < self.b.y {
                Line {
                    a: self.a,
                    b: self.b,
                }
            } else {
                Line {
                    a: self.b,
                    b: self.a,
                }
            }
        }
    }
}

struct Wire {
    lines: Vec<Line>,
}

impl Wire {
    pub fn new(steps: Vec<&str>) -> Wire {
        let mut lines: Vec<Line> = Vec::new();
        let mut previous = Point { x: 0, y: 0 };

        for step in steps {
            let direction = step.get(0..1).unwrap();
            let distance = step.get(1..).unwrap().parse::<i32>().unwrap();

            let next = match direction {
                "R" => Some(Point {
                    x: previous.x + distance,
                    y: previous.y,
                }),
                "D" => Some(Point {
                    x: previous.x,
                    y: previous.y - distance,
                }),
                "L" => Some(Point {
                    x: previous.x - distance,
                    y: previous.y,
                }),
                "U" => Some(Point {
                    x: previous.x,
                    y: previous.y + distance,
                }),
                _ => None,
            }
            .unwrap();

            lines.push(Line {
                a: previous,
                b: next,
            });
            previous = next;
        }

        Wire { lines: lines }
    }
}

fn intersection(line1: &Line, line2: &Line) -> Option<Point> {
    if line1.is_horizontal() && line2.is_vertical() {
        if line2.normalized().a.y < line1.a.y
            && line2.normalized().b.y > line1.b.y
            && line1.normalized().a.x < line2.a.x
            && line1.normalized().b.x > line2.b.x
        {
            return Some(Point {
                x: line2.a.x,
                y: line1.a.y,
            });
        }
    }

    if line1.is_vertical() && line2.is_horizontal() {
        return intersection(line2, line1);
    }

    None
}

fn distance(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn main() {
    let wires = fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(|l| l.split(",").collect())
        .map(Wire::new)
        .collect::<Vec<Wire>>();

    let min = iproduct!(wires[0].lines.iter(), wires[1].lines.iter())
        .map(|(a, b)| intersection(a, b))
        .flatten()
        .map(|i| distance(Point { x: 0, y: 0 }, i))
        .min();

    println!("{:?}", min.unwrap());
}
