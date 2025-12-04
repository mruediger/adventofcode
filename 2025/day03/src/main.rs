use std::{cmp, collections::BinaryHeap};
use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn part1() {
    let sum:u32 = std::fs::read_to_string("../input")
        .expect("input file")
        .trim()
        .lines()
        .map(calculate_joltage)
        .sum();

    println!("part 1: {}", sum);
}

fn part2() {
    let sum:u64 = std::fs::read_to_string("../input")
        .expect("input file")
        .trim()
        .lines()
        .map(calculate_joltage_dozen)
        .sum();

    println!("part 2: {}", sum);
}


fn calculate_joltage(bank:&str) -> u32 {
    let batteries:Vec<u32> = bank.chars().map(|s| s.to_digit(10).expect("a number")).collect();
    let mut max = 0;

    for x in 0..batteries.len() {
        for y in x+1..batteries.len() {
            max = cmp::max(max, batteries[x]*10 + batteries[y]);
        }
    }

    max
}

fn calculate_joltage_dozen_old(bank:&str) -> u64 {
    bank.chars()
        .map(|s| s.to_digit(10).expect("a number"))
        .map(u64::from)
        .combinations(12)
        .map(|c| c.iter().fold(0, |acc, i| acc * 10 + i))
        .max().expect("reason")
}

fn calculate_joltage_dozen(bank:&str) -> u64 {
    let mut batteries:Vec<(usize,u32)> = bank.chars()
        .map(|s| s.to_digit(10).expect("a number"))
        .enumerate()
        .collect();

    batteries.sort_by(|a, b| {
        let second_cmp = b.1.cmp(&a.1);

        if second_cmp == std::cmp::Ordering::Equal {
            b.0.cmp(&a.0)
        } else {
            second_cmp
        }
    });
    println!("{:?}", batteries);

    let mut biggest_dozen:Vec<(usize,u32)> = batteries.into_iter().take(12).collect();
    println!("{:?}", biggest_dozen);
    biggest_dozen.sort_by(|a, b| a.0.cmp(&b.0));

    println!("{:?}", biggest_dozen);

    biggest_dozen.into_iter()
        .map(|(_,v)| u64::from(v))
        .fold(0, |acc, i| acc * 10 + i)
}


#[test]
fn test_part1() {
    assert_eq!(calculate_joltage("987654321111111"), 98);
    assert_eq!(calculate_joltage("811111111111119"), 89);
    assert_eq!(calculate_joltage("234234234234278"), 78);
    assert_eq!(calculate_joltage("818181911112111"), 92);
}

#[test]
fn test_part2() {
    let matrix:Vec<(&str,u64)> = vec![
        ("987654321111111",987654321111),
        ("811111111111119",811111111119),
        ("234234234234278",434234234278),
        ("818181911112111",888911112111)
    ];
    for (input, result) in matrix {
        assert_eq!(calculate_joltage_dozen(input), result);
    }
}
