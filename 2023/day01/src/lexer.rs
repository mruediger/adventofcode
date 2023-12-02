const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(u32),
    String(String),
}

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let next_char = self.chars.next()?;
        match next_char {
            '0'..='9' => next_char.to_digit(10).map(Token::Number),
            'a'..='z' => {
                let mut string = String::from(next_char);
                while let Some(next_char) = self.chars.clone().next() {
                    if next_char.is_alphabetic() {
                        string.push(next_char);
                        self.chars.next();
                        for n in 0..9 {
                            if string.ends_with(DIGITS[n]) {
                                return Some(Token::Number(n as u32 + 1));
                            }
                        }
                    } else {
                        break;
                    }
                }
                Some(Token::String(string))
            }
            _ => None,
        }
    }
}
