use rayon::prelude::*;

mod common;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|s| s.split_once(' '))
        .collect::<Vec<_>>()
        .par_iter()
        .map(|(p, r)| common::permutate_and_match(p.to_string(), r.to_string()))
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|s| s.split_once(' '))
        .map(|(p, r)| {
            p.to_string().push('?');
            (p.repeat(5), r.repeat(5))
        })
        .collect::<Vec<_>>()
        .par_iter()
        .map(|(p, r)| common::permutate_and_match(p.to_string(), r.to_string()))
        .sum()
}
