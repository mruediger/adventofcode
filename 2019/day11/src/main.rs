extern crate intcode;
use intcode::{Program, ReturnCode};
use std::cmp;
use std::collections::HashMap;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Turn {
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
}

fn parse_step(position: Position, direction: Direction, iturn: isize) -> (Direction, Position) {
    let turn = if iturn == 0 { Turn::Left } else { Turn::Right };

    match direction {
        Direction::Up => match turn {
            Turn::Left => (
                Direction::Left,
                Position {
                    x: position.x - 1,
                    y: position.y,
                },
            ),
            Turn::Right => (
                Direction::Right,
                Position {
                    x: position.x + 1,
                    y: position.y,
                },
            ),
        },
        Direction::Right => match turn {
            Turn::Left => (
                Direction::Up,
                Position {
                    x: position.x,
                    y: position.y - 1,
                },
            ),
            Turn::Right => (
                Direction::Down,
                Position {
                    x: position.x,
                    y: position.y + 1,
                },
            ),
        },
        Direction::Down => match turn {
            Turn::Left => (
                Direction::Right,
                Position {
                    x: position.x + 1,
                    y: position.y,
                },
            ),
            Turn::Right => (
                Direction::Left,
                Position {
                    x: position.x - 1,
                    y: position.y,
                },
            ),
        },
        Direction::Left => match turn {
            Turn::Left => (
                Direction::Down,
                Position {
                    x: position.x,
                    y: position.y + 1,
                },
            ),
            Turn::Right => (
                Direction::Up,
                Position {
                    x: position.x,
                    y: position.y - 1,
                },
            ),
        },
    }
}

fn run(mut program: Program, start_on_white: bool) -> HashMap<Position, isize> {
    let mut position = Position { x: 0, y: 0 };
    let mut direction = Direction::Up;
    let mut panel: HashMap<Position, isize> = HashMap::new();
    let mut buffer: Vec<isize> = Vec::new();
    let mut first = start_on_white;

    loop {
        match program.run() {
            ReturnCode::Output => {
                buffer.push(program.output().pop().unwrap());
                if buffer.len() == 2 {
                    panel.insert(position.clone(), buffer[0]);
                    let x = parse_step(position, direction, buffer[1]);
                    direction = x.0;
                    position = x.1;
                    buffer.clear();
                }
            }
            ReturnCode::WaitingForInput => match panel.get(&position) {
                Some(color) => program.input(*color),
                None => {
                    if first {
                        first = false;
                        program.input(1)
                    } else {
                        program.input(0)
                    }
                }
            },
            ReturnCode::Halt | ReturnCode::EndOfProgram => break,
        }
    }
    panel
}

fn render(panel: HashMap<Position, isize>) -> String {
    let mut min_x = std::isize::MAX;
    let mut min_y = std::isize::MAX;
    let mut max_x = std::isize::MIN;
    let mut max_y = std::isize::MIN;

    for (position, _) in &panel {
        min_x = cmp::min(position.x, min_x);
        min_y = cmp::min(position.y, min_y);
        max_x = cmp::max(position.x, max_x);
        max_y = cmp::max(position.y, max_y);
    }

    println!("{}", min_x);
    println!("{}", min_y);
    println!("{}", max_x);
    println!("{}", max_y);

    let mut output = String::new();

    for y in min_y..max_y {
        for x in min_x..max_x {
            match panel.get(&Position { x, y }) {
                Some(&color) => {
                    if color == 0 {
                        output.push('█');
                    } else {
                        output.push(' ');
                    }
                }
                None => output.push(' '),
            }
        }
        output.push('\n');
    }

    output
}

fn render_canvas(panel: HashMap<Position, isize>) -> String {
    let width = 160;
    let height = 160;

    let mut canvas = vec![0; width * height];

    for (position, color) in &panel {
        let index = (position.y + height as isize / 2) as usize * width
            + (position.x + width as isize / 2) as usize;
        canvas[index] = *color;
    }

    let mut output = String::new();
    for y in (0..height) {
        for x in (0..width) {
            let color = if canvas[y * x] == 0 { '█' } else { ' ' };
            output.push(color);
        }
        output.push('\n');
    }
    output
}

fn main() {
    let input: Vec<isize> = std::fs::read_to_string("../input.txt")
        .expect("error reading input.txt")
        .split(",")
        .map(|s| s.parse::<isize>())
        .flatten()
        .collect();

    let mut program = Program::new(input.clone(), vec![0]);
    program.return_on_output = true;

    println!("Answer 1: {}", run(program.clone(), false).len());

    print!("Answer 2: \n{}", render_canvas(run(program.clone(), true)));
}
