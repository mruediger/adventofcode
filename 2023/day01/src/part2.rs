const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run(input: &Vec<String>) -> u32 {
    input.iter().flat_map(|s| number(s)).sum::<u32>()
}

pub fn number(line: &String) -> Option<u32> {
    let a = first_number(line);
    let b = last_number(line);

    match (a, b) {
        (Some(x), Some(y)) => Some(x * 10 + y),
        _ => None,
    }
}

fn first_number(line: &String) -> Option<u32> {
    let mut string = String::new();
    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_digit(10);
        } else {
            string.push(c);
            for n in 0..9 {
                if string.ends_with(DIGITS[n]) {
                    return Some(n as u32 + 1);
                }
            }
        }
    }
    Some(0)
}

fn last_number(line: &String) -> Option<u32> {
    let mut string = String::new();
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10);
        } else {
            string.insert(0, c);
            for n in 0..9 {
                if string.starts_with(DIGITS[n]) {
                    return Some(n as u32 + 1);
                }
            }
        }
    }
    Some(0)
}
