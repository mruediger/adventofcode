use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let (instructions, nodes) = crate::common::parse_input(input);

    let starts = nodes
        .keys()
        .filter(|i| i.ends_with('A'))
        .cloned()
        .collect::<Vec<&str>>();

    starts
        .iter()
        .map(|start| {
            crate::common::count_steps(instructions, start, |s: &str| s.ends_with('Z'), &nodes)
        })
        .fold(1, num::integer::lcm)
}

pub fn run_slow(input: &str) -> usize {
    let (instructions, nodes) = crate::common::parse_input(input);
    count_steps(instructions, nodes)
}

fn count_steps(instructions: &str, nodes: HashMap<&str, (&str, &str)>) -> usize {
    let instructions: Vec<char> = instructions.chars().collect();
    let mut next = nodes
        .clone()
        .into_keys()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<_>>();

    let mut end_found = false;
    let mut i = 0;

    while !end_found {
        next = match instructions[i % instructions.len()] {
            'L' => next.iter().map(|n| nodes[n].0).collect::<Vec<_>>(),
            'R' => next.iter().map(|n| nodes[n].1).collect::<Vec<_>>(),
            _ => panic!(
                "unknown instruction found {:?}",
                instructions[i % instructions.len()]
            ),
        };

        end_found = next.iter().all(|s| s.ends_with('Z'));
        i += 1;
    }

    i
}

#[cfg(test)]
mod test {

    const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_get_starts() {
        let (_, nodes) = crate::common::parse_input(INPUT);

        let starts = nodes
            .into_keys()
            .filter(|s| s.ends_with('A'))
            .collect::<Vec<_>>();
        assert!(starts.contains(&"22A"));
        assert!(starts.contains(&"11A"));
    }

    #[test]
    fn count_steps() {
        let (instructions, nodes) = crate::common::parse_input(INPUT);
        println!("{}", super::count_steps(instructions, nodes));
    }
}
