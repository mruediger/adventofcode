use std::collections::HashSet;

#[derive(Default, Debug, Clone)]
struct Card<'a> {
    scratch: HashSet<&'a str>,
    winning: HashSet<&'a str>,
}

impl<'a> Card<'a> {
    fn new(line: &'a str) -> Self {
        let (_, line) = line.split_once(": ").unwrap();
        let (wn_string, on_string) = line.split_once('|').unwrap();
        Card {
            winning: wn_string.split_whitespace().collect(),
            scratch: on_string.split_whitespace().collect(),
        }
    }

    fn count_wins(&self) -> usize {
        self.scratch.intersection(&self.winning).count()
    }
}

fn count_cards(cards: Vec<Card>) -> u32 {
    let mut cards_count = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        for x in (i + 1)..=(i + card.count_wins()) {
            cards_count[x] += cards_count[i];
        }
    }
    cards_count.iter().sum()
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
