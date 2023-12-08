use regex::Regex;
use std::collections::HashMap;

pub fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut iter = input.lines();
    let instructions = iter.next().unwrap().trim();
    let re = Regex::new(r"^([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)$").unwrap();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in iter.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let Some(caps) = re.captures(line) else {
            panic!("error {}", line);
        };

        let (_, [key, left, right]) = caps.extract();
        nodes.insert(key, (left, right));
    }

    (instructions, nodes)
}

pub fn count_steps(
    instructions: &str,
    start: &str,
    is_end: fn(&str) -> bool,
    nodes: &HashMap<&str, (&str, &str)>,
) -> usize {
    let instructions: Vec<char> = instructions.chars().collect();
    let mut end_found = false;
    let mut i = 0;
    let mut next = start;

    while !end_found {
        next = match instructions[i % instructions.len()] {
            'L' => nodes[next].0,
            'R' => nodes[next].1,
            _ => panic!(
                "unknown instruction found {:?}",
                instructions[i % instructions.len()]
            ),
        };
        end_found = is_end(next);
        i += 1;
    }
    i
}

#[cfg(test)]
mod test {

    #[test]
    fn parse_input() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let (instructions, nodes) = super::parse_input(input);
        assert_eq!(instructions, "LLR");
        assert_eq!(nodes["AAA"], ("BBB", "BBB"));
        assert_eq!(nodes["BBB"], ("AAA", "ZZZ"));
        assert_eq!(nodes["ZZZ"], ("ZZZ", "ZZZ"));
    }

    #[test]
    fn loop_instructions() {
        let instructions: Vec<char> = "LLR".chars().collect();

        let mut result = Vec::new();
        for i in 0..10 {
            result.push(instructions[i % instructions.len()]);
        }
        assert_eq!(result.iter().collect::<String>(), "LLRLLRLLRL");
    }

    #[test]
    fn count_steps() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let (instructions, nodes) = super::parse_input(input);
        assert_eq!(
            super::count_steps(instructions, "AAA", |s: &str| s == "ZZZ", &nodes),
            6
        );
    }
}
