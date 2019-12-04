use std::collections::HashSet;
use std::fs;

const INPUT: &str = "../input.txt";

fn parse_step(step: &str) -> impl Iterator<Item = (isize, isize)> {
    let direction = step.get(0..1).unwrap();
    let distance = step.get(1..).unwrap().parse().unwrap();

    let coord = match direction {
        "U" => (0, -1),
        "D" => (0, 1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => (0, 0),
    };

    std::iter::repeat(coord).take(distance)
}

fn parse_line(line: &str) -> impl Iterator<Item = (isize, isize)> + '_ {
    line.split(",")
        .flat_map(parse_step)
        .scan((0, 0), |pos, step| {
            pos.0 += step.0;
            pos.1 += step.1;
            Some(pos.clone())
        })
}

fn manhattan_distance(pos: &(isize, isize)) -> usize {
    pos.0.abs() as usize + pos.1.abs() as usize
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();

    let mut lines = input.lines().map(parse_line);

    let steps_0: Vec<(isize, isize)> = lines.next().unwrap().collect();
    let steps_1: Vec<(isize, isize)> = lines.next().unwrap().collect();

    let positions_0: HashSet<_> = steps_0.iter().cloned().collect();
    let positions_1: HashSet<_> = steps_1.iter().cloned().collect();
    let collisions = positions_0.intersection(&positions_1);

    let closest_collision = collisions.clone().map(manhattan_distance).min();
    println!("Part One: {:?}", closest_collision.unwrap());

    let quickest_collision = collisions
        .map(|pos| {
            steps_0.iter().position(|x| x == pos).unwrap()
                + steps_1.iter().position(|x| x == pos).unwrap()
                + 2
        })
        .min();
    println!("Part Two: {:?}", quickest_collision.unwrap());
}
