fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    input.split(',').map(|s| hash(s.trim())).sum()
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
}
