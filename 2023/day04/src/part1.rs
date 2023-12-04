use {once_cell::sync::Lazy, regex::Regex};

fn parse_line(s: &str) -> u32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Card *([0-9]+): +(.+) \| +(.+)$").unwrap());

    let Some(caps) = RE.captures(s) else {
        println!("error {}", s);
        return 0;
    };
    
    let (_, [_, wn_string, on_string]) = caps.extract();
    let parse = |s: &str| s.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let wn: Vec<u32> = parse(wn_string);
    let on: Vec<u32> = parse(on_string);
    let count = on.iter().filter(|n| wn.contains(n)).count() as u32;

    if count > 0 {
        2u32.pow(count.saturating_sub(1) as u32)
    }else {
        0
    }
}

pub fn run(input: &Vec<String>) -> u32 {
    input.iter().map(|l| parse_line(l)).sum()
}

#[test]
fn test_parse_line() {
    let input = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    assert_eq!(
        input.iter().map(|l| parse_line(l)).collect::<Vec<_>>(),
        vec![8, 2, 2, 1, 0, 0]
    );
}
