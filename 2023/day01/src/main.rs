mod lexer;

use crate::lexer::Lexer;
use crate::lexer::Token;

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

fn part2(input: &Vec<String>) -> u32 {
    let mut sum = 0;
    for line in input {
        let mut lexer = Lexer::new(line);
        let tokens = lexer.tokenize();

        let mut first: u32 = 0;
        let mut last: u32 = 0;

        for token in tokens.iter() {
            match token {
                Token::Number(x) => {
                    first = *x;
                    break;
                }
                _ => continue,
            }
        }

        for token in tokens.iter().rev() {
            match token {
                Token::Number(x) => {
                    last = *x;
                    break;
                }
                _ => continue,
            }
        }

        println!("{}", line);
        println!("{:?}", tokens);
        println!("{}", first * 10 + last);

        sum += first * 10 + last;
    }

    sum
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

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("xtwo1nineee");
    let tokens = lexer.tokenize();
    assert_eq!(
        tokens,
        vec![Token::Number(2), Token::Number(1), Token::Number(9),]
    );
}

#[test]
fn test_lexer_complicated() {
    let mut lexer = Lexer::new("ninezfzseveneight5kjrjvtfjqt5nineone");
    assert_eq!(
        lexer.tokenize(),
        vec![
            Token::Number(9),
            Token::String("fz".to_string()),
            Token::Number(7),
            Token::Number(8),
            Token::Number(5),
            Token::String("kjrjvtfjqt".to_string()),
            Token::Number(5),
            Token::Number(1)
        ]
    )
}
