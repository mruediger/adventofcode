fn main() {
     let input: Vec<String> = std::fs::read_to_string("../input")
        .unwrap()
        .lines()
        .map(String::from)
         .collect();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn part1(input: Vec<String>) -> i32 {
    let mut status = Some(50);
    let mut result = 0;
    for turn in input {
        status = turn_dial(status.unwrap(), turn.clone());
        if status == Some(0) {
            result += 1;
        };
    }
    result
}

fn part2(input: Vec<String>) -> i32 {
    let mut status = Some(50);
    let mut result = 0;
    for turn in input {
        let (status_mod, result_mod) = turn_dial_count_rotations(status.unwrap(), turn).unwrap();
        result += result_mod;
        status = Some(status_mod);
    }
    result
}


fn turn_dial(start: i32, turn:String) -> Option<i32> {
    match turn.chars().next() {
        Some('L') => Some(turn_left(start, turn.strip_prefix("L")?.parse().unwrap()).0),
        Some('R') => Some(turn_right(start, turn.strip_prefix("R")?.parse().unwrap()).0),
        Some(_) => None,
        None => None
    }
}

fn turn_right(start: i32, turn: i32) -> (i32,i32) {
    let sum = start + turn;
    let result = sum % 100;
    let rotations = sum / 100;

    (result, rotations)
}

fn turn_left(start: i32, turn: i32) -> (i32,i32) {
    let result = (start + 100 - (turn % 100)) % 100;
    let rotations = if start == 0 {
        turn / 100
    } else {
        ((start - (turn + 100)) / 100).abs()
    };

    (result, rotations)
}


fn turn_dial_count_rotations(start: i32, turn:String) -> Option<(i32,i32)> {
    match turn.chars().next() {
        Some('L') => Some(turn_left(start, turn.strip_prefix("L")?.parse().unwrap())),
        Some('R') => Some(turn_right(start, turn.strip_prefix("R")?.parse().unwrap())),
        Some(_) => None,
        None => None
    }
}

#[test]
fn test_turn1() {
    assert_eq!(turn_dial(50, "L68".to_string()), Some(82));
    assert_eq!(turn_dial(82, "L30".to_string()), Some(52));
    assert_eq!(turn_dial(52, "R48".to_string()), Some(0));
    assert_eq!(turn_dial(0, "L5".to_string()), Some(95));
    assert_eq!(turn_dial(95, "R60".to_string()), Some(55));
    assert_eq!(turn_dial(55, "L55".to_string()), Some(0));
    assert_eq!(turn_dial(0, "L1".to_string()), Some(99));
    assert_eq!(turn_dial(99, "L99".to_string()), Some(0));
    assert_eq!(turn_dial(0, "R14".to_string()), Some(14));
    assert_eq!(turn_dial(14, "L82".to_string()), Some(32));
}

#[test]
fn test_part1() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    assert_eq!(part1(input), 1165);
}

#[test]
fn test_turn2() {
    assert_eq!(turn_dial_count_rotations(50, "L68".to_string()), Some((82,1)));
    assert_eq!(turn_dial_count_rotations(82, "L30".to_string()), Some((52,0)));
    assert_eq!(turn_dial_count_rotations(52, "R48".to_string()), Some((0,1)));
    assert_eq!(turn_dial_count_rotations(0, "L5".to_string()), Some((95,0)));
    assert_eq!(turn_dial_count_rotations(95, "R60".to_string()), Some((55,1)));
    assert_eq!(turn_dial_count_rotations(55, "L55".to_string()), Some((0,1)));
    assert_eq!(turn_dial_count_rotations(0, "L1".to_string()), Some((99,0)));
    assert_eq!(turn_dial_count_rotations(99, "L99".to_string()), Some((0,1)));
    assert_eq!(turn_dial_count_rotations(0, "R14".to_string()), Some((14,0)));
    assert_eq!(turn_dial_count_rotations(14, "L82".to_string()), Some((32,1)));
}
