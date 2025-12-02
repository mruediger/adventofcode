fn main() {
    let sum_part1 = std::fs::read_to_string("../input")
        .expect("a file")
        .trim()
        .split(',')
        .flat_map(|range| find_invalid(range, is_not_repeated))
        .map(|s| s.parse::<i64>().expect("an int"))
        .sum::<i64>();
    println!("part 1: {}", sum_part1);

    let sum_part2 = std::fs::read_to_string("../input")
        .expect("a file")
        .trim()
        .split(',')
        .flat_map(|range| find_invalid(range, is_not_repeated_multiple))
        .map(|s| s.parse::<i64>().expect("an int"))
        .sum::<i64>();
    println!("part 2: {}", sum_part2);
}

fn is_not_repeated(id:&str) -> bool {
    (! id.len().is_multiple_of(2)) |  (id[.. id.len() / 2] != id[id.len() /2 ..])
}

fn is_not_repeated_multiple(id:&str) -> bool {
    for i in (1..=(id.len() / 2)).rev() {
        let chars : Vec<char> = id.chars().collect::<Vec<char>>();
        let chunks : Vec<String> = chars
            .chunks(i)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>();

        let first = &chunks[0];
        if chunks.iter().all(|chunk| chunk == first) {
            return false
        }
    }

    true
}

fn find_invalid(range:&str, filter: fn(id: &str) -> bool) -> Vec<String>{
    let indices :Vec<u64> = range
        .split('-')
        .map(|s| s.parse().expect("an usize"))
        .collect();

    (indices[0]..=indices[1])
        .map(|i| i.to_string())
        .filter(|s| ! filter(s))
        .collect()
}

//#[test]
fn test_valid() {
    assert!(! is_not_repeated("1111"));
    assert!(! is_not_repeated("2222"));
    assert!(! is_not_repeated("99"));
    assert!(is_not_repeated("115"));

    assert_eq!(find_invalid("11-22", is_not_repeated), ["11","22"]);
    assert_eq!(find_invalid("998-1012", is_not_repeated), ["1010"]);
    assert_eq!(find_invalid("1188511880-1188511890", is_not_repeated), ["1188511885"]);
    assert_eq!(find_invalid("1698522-1698528", is_not_repeated), Vec::<String>::new())
}

#[test]
fn test_repeated_multiple() {
    assert_eq!(find_invalid("11-22", is_not_repeated_multiple), ["11","22"]);
    assert_eq!(find_invalid("95-115", is_not_repeated_multiple), ["99","111"]);
    assert_eq!(find_invalid("998-1012", is_not_repeated_multiple), ["999", "1010"]);
    assert_eq!(find_invalid("1188511880-1188511890", is_not_repeated_multiple), ["1188511885"]);
    assert_eq!(find_invalid("1698522-1698528", is_not_repeated_multiple), Vec::<String>::new())
}
