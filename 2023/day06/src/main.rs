mod common;
mod part1;
mod part2;

fn parse_input(input: String) -> (Vec<u64>, Vec<u64>) {
    let lines: Vec<_> = input.lines().filter(|s| !s.is_empty()).collect();

    let times: Vec<u64> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<u64> = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (times, distances)
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let (times, distances) = parse_input(input);
    println!("Part 1 {}", part1::run(&times, &distances));
    println!("Part 2 {}", part2::run(&times, &distances));
}

#[test]
fn test_parse_input() {
    let input = "Time:      7  15   30
Distance:  9  40  200";

    let (times, distances) = parse_input(input.to_string());

    assert_eq!(times, [7, 15, 30]);
    assert_eq!(distances, [9, 40, 200]);
}
