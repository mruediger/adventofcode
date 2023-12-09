fn main() {
    let input = std::fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();

    println!("Part 1: {}", part1_run(&input));
    println!("Part 2: {}", part2_run(&input));
}

fn part1_run(sequences: &[Vec<i32>]) -> i32 {
    sequences.iter().map(|sequence| next(sequence)).sum()
}

fn part2_run(sequences: &[Vec<i32>]) -> i32 {
    sequences.iter().map(|sequence| prev(sequence)).sum()
}

fn next(sequence: &[i32]) -> i32 {
    if sequence.iter().all(|i| i == &0) {
        0
    } else {
        sequence.last().unwrap() + next(&diff(sequence))
    }
}

fn prev(sequence: &[i32]) -> i32 {
    if sequence.iter().all(|i| i == &0) {
        0
    } else {
        sequence.first().unwrap() - prev(&diff(sequence))
    }
}

fn diff(sequence: &[i32]) -> Vec<i32> {
    sequence.windows(2).map(|x| x[1] - x[0]).collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn next() {
        assert_eq!(super::next(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(super::next(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(super::next(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn prev() {
        assert_eq!(super::prev(&[10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn diff() {
        assert_eq!(super::diff(&[0, 3, 6, 9, 12, 15]), vec![3, 3, 3, 3, 3])
    }
}
