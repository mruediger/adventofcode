pub mod lexer;

const INPUT: &str = "../input.txt";

struct Line<'a> {
    s: &'a str,
}

impl Line<'_> {
    pub fn first_number(&self) -> Option<u32> {
        self.s
            .chars()
            .find(|c| c.is_ascii_digit())
            .unwrap_or_default()
            .to_digit(10)
    }

    pub fn last_number(&self) -> Option<u32> {
        self.s
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap_or_default()
            .to_digit(10)
    }

    pub fn number(&self) -> Option<u32> {
        let a = self.first_number();
        let b = self.last_number();

        match (a, b) {
            (Some(x), Some(y)) => Some(x * 10 + y),
            _ => None,
        }
    }
}

fn part1(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|s| Line { s })
        .flat_map(|n| n.number())
        .sum::<u32>()
}

fn part2(_input: &Vec<String>) -> u32 {
    0
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
