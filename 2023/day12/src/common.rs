use regex::Regex;

pub fn permutate(input: String) -> Vec<String> {
    let mut stack: Vec<String> = vec![input];
    let mut solution: Vec<String> = Vec::new();

    while let Some(s) = stack.pop() {
        for c in [".", "#"] {
            let tmp = s.replacen('?', c, 1);
            if tmp.contains('?') {
                stack.push(tmp);
            } else {
                solution.push(tmp)
            }
        }
    }
    solution
}

pub fn match_patterns(input: Vec<String>, pattern: &str) -> usize {
    let mut regex = pattern
        .split(',')
        .map(|i| format!("#{{{}}}", i))
        .collect::<Vec<String>>()
        .join("\\.+");

    regex.insert_str(0, "^[^#]*");
    regex.push_str("[^#]*$");

    let re = Regex::new(&regex).unwrap();
    let mut count = 0;
    for s in input {
        if re.is_match(&s) {
            count += 1;
        }
    }
    count
}

pub fn permutate_and_match(input: String, pattern: String) -> usize {
    let mut regex = pattern
        .split(',')
        .map(|i| format!("#{{{}}}", i))
        .collect::<Vec<String>>()
        .join("\\.+");

    regex.insert_str(0, "^[^#]*");
    regex.push_str("[^#]*$");
    let re = Regex::new(&regex).unwrap();

    let mut count = 0;
    let mut stack: Vec<String> = vec![input.to_string()];
    while let Some(s) = stack.pop() {
        for c in [".", "#"] {
            let tmp = s.replacen('?', c, 1);
            if tmp.contains('?') {
                stack.push(tmp);
            } else if re.is_match(&tmp) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {

    use indoc::indoc;
    use rayon::prelude::*;

    const INPUT: &str = indoc! {"???.### 1,1,3
                                 .??..??...?##. 1,1,3
                                 ?#?#?#?#?#?#?#? 1,3,1,6
                                 ????.#...#... 4,1,1
                                 ????.######..#####. 1,6,5
                                 ?###???????? 3,2,1"};
    #[test]
    fn permutate() {
        assert_eq!(
            super::permutate(".??..??...?##.".to_string()).len(),
            2_usize.pow(5)
        );
    }

    #[test]
    fn match_pattern() {
        assert_eq!(
            super::match_patterns(super::permutate("?#?#?#?#?#?#?#??".to_string()), "1,3,1,6"),
            1
        );
    }

    #[test]
    fn match_all_patterns() {
        let count = INPUT
            .lines()
            .flat_map(|s| s.split_once(' '))
            .map(|(p, r)| super::permutate_and_match(p.to_string(), r.to_string()))
            .collect::<Vec<_>>();

        assert_eq!(count, vec![1, 4, 1, 1, 4, 10]);
    }

    #[test]
    fn match_all_patterns_extended() {
        let count = INPUT
            .lines()
            .flat_map(|s| s.split_once(' '))
            .map(|(p, r)| {
                p.to_string().push('?');
                (p.repeat(5), r.repeat(5))
            })
            .collect::<Vec<_>>()
            .par_iter()
            .map(|(p, r)| super::permutate_and_match(p.to_string(), r.to_string()))
            .collect::<Vec<_>>();

        assert_eq!(count, vec![1, 4, 1, 1, 4, 10]);
    }
}
