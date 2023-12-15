use {once_cell::sync::Lazy, regex::Regex};

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input.split(',').map(|s| hash(s.trim())).sum()
}

fn part2(input: &str) -> usize {
    let mut lens_config: Vec<Vec<(String, usize)>> = Vec::new();
    for _ in 0..256 {
        lens_config.push(Vec::new());
    }

    for (label, operation, focal_length) in input.split(',').map(parse_step) {
        match operation {
            "=" => insert(
                &mut lens_config[hash(label)],
                label.to_string(),
                focal_length.parse().unwrap(),
            ),
            "-" => remove(&mut lens_config[hash(label)], label.to_string()),
            _ => todo!(),
        };
    }

    lens_config
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(s, (_, focal_length))| (i + 1) * (s + 1) * focal_length)
                .sum::<usize>()
        })
        .sum()
}

fn insert(lens_box: &mut Vec<(String, usize)>, label: String, focal_length: usize) {
    for (i, (l, _)) in lens_box.clone().iter().enumerate() {
        if *l == label {
            lens_box[i] = (label, focal_length);
            return;
        }
    }
    lens_box.push((label, focal_length))
}

fn remove(lens_box: &mut Vec<(String, usize)>, label: String) {
    for (i, (l, _)) in lens_box.clone().iter().enumerate() {
        if *l == label {
            lens_box.remove(i);
            return;
        }
    }
}

fn hash(input: &str) -> usize {
    let mut current = 0;

    for c in input.chars() {
        current += c as usize;
        current *= 17;
        current %= 256;
    }

    current
}

fn parse_step(step: &str) -> (&str, &str, &str) {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z]+)([=-])([0-9]*)").unwrap());

    let Some(caps) = RE.captures(step) else {
        panic!("error {}", step);
    };

    let (_, [label, operation, focal_length]) = caps.extract();

    (label, operation, focal_length)
}

#[cfg(test)]
mod test {

    #[test]
    fn hash_string() {
        assert_eq!(super::hash("HASH"), 52);
    }

    #[test]
    fn part1() {
        assert_eq!(
            super::part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn parse_steps() {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            .split(',')
            .map(super::parse_step)
            .for_each(|(label, operation, focal_length)| {
                println!("{} {} {}", label, operation, focal_length)
            });
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
