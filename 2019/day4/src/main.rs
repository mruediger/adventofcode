#[test]
fn test_digits() {
    assert_eq!(digits(123000), [1, 2, 3, 0, 0, 0]);
}

fn digits(num: usize) -> Vec<usize> {
    if num == 0 {
        return vec![0];
    }

    let mut x = num;
    let mut result = std::iter::from_fn(|| {
        if x == 0 {
            None
        } else {
            let current = x % 10;
            x /= 10;
            Some(current)
        }
    })
    .collect::<Vec<usize>>();

    result.reverse();
    result
}

fn never_decreases(i: usize) -> bool {
    digits(i).windows(2).all(|c| c[0] <= c[1])
}

fn has_double(i: usize) -> bool {
    let d = digits(i);

    for i in 1..d.len() {
        if d[i] == d[i - 1] {
            return true;
        }
    }

    false
}

#[test]
fn test_double_strict() {
    assert_eq!(has_double_strict(112233), true);
    assert_eq!(has_double_strict(111333), false);
    assert_eq!(has_double_strict(111122), true);

    // this would be wrong but is filtered out by never_decreases()
    assert_eq!(has_double_strict(122221), true);
}

fn has_double_strict(i: usize) -> bool {
    let d = digits(i);

    for i in 0..d.len() {
        let mut c = 0;

        for j in 0..d.len() {
            if d[i] == d[j] {
                c += 1
            }
        }
        if c == 2 {
            return true;
        }
    }
    false
}

fn main() {
    let mut pin1_count = 0;
    let mut pin2_count = 0;

    for i in 347312..805915 {
        if has_double(i) && never_decreases(i) {
            pin1_count += 1;

            if has_double_strict(i) {
                pin2_count += 1
            }
        }
    }

    println!("{}", pin1_count);
    println!("{}", pin2_count);
}
