use std::cmp;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let patterns = parse(input);
    patterns.iter().map(|p| p.fold()).sum()
}

fn part2(input: &str) -> usize {
    let patterns = parse(input);
    patterns.iter().map(|p| p.fold_smudged()).sum()
}

#[derive(Debug)]
struct Pattern {
    field: Vec<Vec<char>>,
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in &self.field {
            for c in line {
                write!(f, "{}", c)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Pattern {
    fn rows(&self) -> Vec<Vec<char>> {
        let mut result = Vec::new();
        for row in &self.field {
            result.push(row.clone());
        }
        result
    }

    fn collumns(&self) -> Vec<Vec<char>> {
        if self.field.is_empty() {
            return Vec::new();
        }
        let mut result = vec![Vec::new(); self.field[0].len()];

        for row in &self.field {
            for (i, c) in row.iter().enumerate() {
                result[i].push(*c);
            }
        }

        result
    }

    fn fold(&self) -> usize {
        self.collumn_fold() + self.row_fold() * 100
    }

    fn collumn_fold(&self) -> usize {
        let collumns = self.collumns();
        let mut fold = 0;
        for start in find_fold_start(&collumns) {
            if let Some(tmp) = check_fold(start, &collumns) {
                if tmp > start {
                    fold = tmp;
                }
            }
        }

        fold
    }

    fn row_fold(&self) -> usize {
        let rows = self.rows();
        let mut fold = 0;
        for start in find_fold_start(&rows) {
            if let Some(tmp) = check_fold(start, &rows) {
                if tmp > start {
                    fold = tmp;
                }
            }
        }

        fold
    }

    fn fold_smudged(&self) -> usize {
        cmp::max(self.collumn_fold_smudged(), self.row_fold_smudged() * 100)
    }

    fn collumn_fold_smudged(&self) -> usize {
        let collumns = self.collumns();
        for start in find_fold_start(&collumns) {
            if let Some(fold) = check_fold_smudged(start, &collumns) {
                return fold;
            }
        }

        0
    }

    fn row_fold_smudged(&self) -> usize {
        let rows = self.rows();
        for start in find_fold_start_smudged(&rows) {
            if let Some(fold) = check_fold_smudged(start, &rows) {
                return fold;
            }
        }

        0
    }
}

fn find_fold_start(patterns: &Vec<Vec<char>>) -> Vec<usize> {
    let mut start = Vec::new();
    for (i, pair) in patterns.windows(2).enumerate() {
        if pair[0] == pair[1] {
            start.push(i);
        }
    }
    start
}

fn find_fold_start_smudged(patterns: &Vec<Vec<char>>) -> Vec<usize> {
    let mut start = Vec::new();
    for (i, pair) in patterns.windows(2).enumerate() {
        if smudge_eq(&pair[0], &pair[1]) {
            start.push(i);
        }
    }
    start
}

fn check_fold(start: usize, patterns: &Vec<Vec<char>>) -> Option<usize> {
    let mut i = 0;
    while start as i32 - i as i32 >= 0 && start + i + 1 < patterns.len() {
        if patterns[start - i] != patterns[start + i + 1] {
            //        println!("{:?}", patterns[start - i]);
            //       println!("{:?}", patterns[start + i + 1]);
            return None;
        }
        i += 1;
    }

    Some(start + 1)
}

fn check_fold_smudged(start: usize, patterns: &Vec<Vec<char>>) -> Option<usize> {
    let mut i = 0;
    let mut differences = 0;
    while start as i32 - i as i32 >= 0 && start + i + 1 < patterns.len() {
        if !smudge_eq(&patterns[start - i], &patterns[start + i + 1]) || differences > 1 {
            return None;
        } else if patterns[start - i] != patterns[start + i + 1] {
            differences += 1;
        }
        i += 1;
    }

    Some(start + 1)
}

fn parse(input: &str) -> Vec<Pattern> {
    let mut field = Vec::new();
    let mut patterns = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Pattern { field });
            field = Vec::new();
        } else {
            field.push(line.chars().collect());
        }
    }

    patterns.push(Pattern { field });
    patterns
}

fn smudge_eq(a: &Vec<char>, b: &Vec<char>) -> bool {
    if a == b {
        return true;
    }

    let mut differences = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            differences += 1;
        }
        if differences > 1 {
            return false;
        }
    }

    differences <= 1
}

#[cfg(test)]
mod test {

    use indoc::indoc;
    const INPUT: &str = indoc! {"#.##..##.
                                 ..#.##.#.
                                 ##......#
                                 ##......#
                                 ..#.##.#.
                                 ..##..##.
                                 #.#.##.#.
                                 
                                 #...##..#
                                 #....#..#
                                 ..##..###
                                 #####.##.
                                 #####.##.
                                 ..##..###
                                 #....#..#"};

    #[test]
    fn parse() {
        let patterns = super::parse(INPUT);
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns.iter().map(|p| p.field.len()).sum::<usize>(), 14);
    }

    #[test]
    fn fold() {
        let patterns = super::parse(INPUT);
        assert_eq!(patterns[0].collumn_fold(), 5);
        assert_eq!(patterns[0].row_fold(), 0);
        assert_eq!(patterns[1].collumn_fold(), 0);
        assert_eq!(patterns[1].row_fold(), 4);
    }

    #[test]
    fn run() {
        assert_eq!(super::part1(INPUT), 405);
    }

    #[test]
    fn smudge_eq() {
        let a = "#.##..##.".chars().collect();
        let b = "..##..##.".chars().collect();
        assert!(super::smudge_eq(&a, &b));
    }

    #[test]
    fn run_smudged() {
        assert_eq!(super::part2(INPUT), 400);
    }
}
