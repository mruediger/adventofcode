use {once_cell::sync::Lazy, regex::Regex};

#[derive(Default, Debug, Clone)]
struct Card {
    number: usize,
    scratch: Vec<u32>,
    winning: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Self {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^Card *([0-9]+): +(.+) \| +(.+)$").unwrap());

        let Some(caps) = RE.captures(line) else {
            println!("error {}", line);
            return Card::default();
        };

        let (_, [number, wn_string, on_string]) = caps.extract();
        let parse = |s: &str| s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Card {
            number: number.parse().unwrap(),
            winning: parse(wn_string),
            scratch: parse(on_string),
        }
    }

    fn count_wins(&self) -> usize {
        self.scratch
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

fn count_cards(cards: Vec<Card>) -> u32 {
    let mut queue = cards.clone();
    let mut count = 0;

    while let Some(card) = queue.pop() {
        let wins = card.count_wins();
        count += 1;
        cards[card.number..(card.number + wins)]
            .iter()
            .for_each(|c| queue.push(c.clone()));
    }

    count
}

pub fn run(input: &Vec<String>) -> u32 {
    let cards: Vec<Card> = input.iter().map(|s| Card::new(s)).collect();
    count_cards(cards)
}

#[test]
fn test() {
    let input = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    let cards: Vec<Card> = input.iter().map(|s| Card::new(s)).collect();
    assert_eq!(count_cards(cards), 30);
}
