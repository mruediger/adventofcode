mod common;
mod part1;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut galaxy = crate::common::parse(input);
    galaxy.expand(1);
    galaxy
        .pairs()
        .iter()
        .map(|(a, b)| galaxy.distance(a, b))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut galaxy = crate::common::parse(input);
    galaxy.expand(1000000);
    galaxy
        .pairs()
        .iter()
        .map(|(a, b)| galaxy.distance(a, b))
        .sum()
}
