fn main() {
    let input = std::fs::read_to_string("../input")
        .expect("input file");

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}


fn parse(input:&str) -> (Vec<(u64,u64)>,Vec<u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(u64,u64)> = ranges.lines().map(|s| {
        let (first,last) = s.split_once("-").unwrap();
        (first.parse().unwrap(), last.parse().unwrap())
    }).collect();

    let ingredients: Vec<u64> = ingredients.lines().map(|s| s.parse().unwrap()).collect();

    (ranges,ingredients)
}

fn part1(input:&str) -> usize {
    let (ranges, ingredients) = parse(input);

    ingredients
        .iter()
        .filter(|id| ranges.iter().any(|(start,end)| *id >= start && *id <= end))
        .count()
}

fn part2(input:&str) -> usize {
    let (mut ranges,_) = parse(input);

    ranges.sort_by_key(|(start,_)| *start);

    let mut merged:Vec<(u64,u64)> = vec![*ranges.first().unwrap()];

    for range in ranges.iter().skip(1) {
        let prev = *merged.last().unwrap();
        if range.0 > prev.1 {
            merged.push(*range);
        } else {
            merged.pop();
            merged.push((prev.0, prev.1.max(range.1)))
        }
    }

    merged.iter().flat_map(|(start,end)| *start..=*end).count()
}

mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn part1() {
        assert_eq!(super::part1(CASE), 3);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(CASE), 14);
    }
}
