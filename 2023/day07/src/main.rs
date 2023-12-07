mod part1;
mod part2;

fn main() {
    let input: Vec<String> = std::fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("Part1: {}", part1::run(&input));
    println!("Part2: {}", part2::run(&input));
}
