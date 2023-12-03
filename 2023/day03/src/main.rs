mod part1;
mod part2;

fn main() {
    let input: Vec<_> = std::fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    println!("Part1: {}", part1::run(&input));
    println!("Part1: {}", part2::run(&input));
}
