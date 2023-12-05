mod common;
mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("{}", part1::run(&input));
    println!("{}", part2::run(&input));
}
