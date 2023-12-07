use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    winnings: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl<'a> Hand<'a> {
    fn new(string: &'a str) -> Self {
        let parts = string.split_whitespace().collect::<Vec<_>>();

        Hand {
            cards: parts[0],
            winnings: parts[1].parse().unwrap(),
        }
    }

    fn rank(&self) -> HandType {
        let mut chars = self.cards.chars().collect::<Vec<_>>();
        chars.sort();

        let mut equals = 0;
        let mut rank = HandType::HighCard;
        let mut iter = chars.iter().peekable();

        while {
            let c = iter.next();
            let n = iter.peek();

            if c == n.copied() {
                equals += 1;
            } else {
                rank = match (equals, rank) {
                    (0, HandType::HighCard) => HandType::HighCard,
                    (1, HandType::HighCard) => HandType::OnePair,
                    (1, HandType::OnePair) => HandType::TwoPair,
                    (1, HandType::ThreeOfAKind) => HandType::FullHouse,
                    (2, HandType::HighCard) => HandType::ThreeOfAKind,
                    (2, HandType::OnePair) => HandType::FullHouse,
                    (3, HandType::HighCard) => HandType::FourOfAKind,
                    (4, HandType::HighCard) => HandType::FiveOfAKind,
                    (_, _) => rank,
                };

                equals = 0;
            }
            c.is_some()
        } {}
        rank
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Hand) -> Ordering {
        let s_rank = self.rank();
        let o_rank = other.rank();

        if s_rank == o_rank {
            let map_order = |c: char| match c {
                'A' => 'c',
                'K' => 'b',
                'Q' => 'a',
                'J' => '9',
                'T' => '8',
                '9' => '7',
                '8' => '6',
                '7' => '5',
                '6' => '4',
                '5' => '3',
                '4' => '2',
                '3' => '1',
                '2' => '0',
                _ => panic!("unknown char"),
            };

            let s_chars = self.cards.chars().map(map_order).collect::<String>();
            let o_chars = other.cards.chars().map(map_order).collect::<String>();

            s_chars.cmp(&o_chars)
        } else {
            s_rank.cmp(&o_rank)
        }
    }
}

pub fn run(input: &[String]) -> u32 {
    let mut hands = input.iter().map(|s| Hand::new(s)).collect::<Vec<_>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.winnings * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rank() {
        assert_eq!(Hand::new("32T3K 0").rank(), HandType::OnePair);
        assert_eq!(Hand::new("T55J5 0").rank(), HandType::ThreeOfAKind);
        assert_eq!(Hand::new("KK677 0").rank(), HandType::TwoPair);
        assert_eq!(Hand::new("KTJJT 0").rank(), HandType::TwoPair);
        assert_eq!(Hand::new("QQQJA 0").rank(), HandType::ThreeOfAKind);
    }

    #[test]
    fn compare() {
        let mut hands = [
            Hand::new("32T3K 0"),
            Hand::new("T55J5 0"),
            Hand::new("KK677 0"),
            Hand::new("KTJJT 0"),
            Hand::new("QQQJA 0"),
        ];
        hands.sort();

        let expected = [
            Hand::new("32T3K 0"),
            Hand::new("KTJJT 0"),
            Hand::new("KK677 0"),
            Hand::new("T55J5 0"),
            Hand::new("QQQJA 0"),
        ];

        assert_eq!(hands, expected);
    }

    #[test]
    fn total_winnings() {
        let mut hands = [
            Hand::new("32T3K 765"),
            Hand::new("T55J5 684"),
            Hand::new("KK677 28"),
            Hand::new("KTJJT 220"),
            Hand::new("QQQJA 483"),
        ];
        hands.sort();

        let mut sum = 0;

        for (i, hand) in hands.iter().enumerate() {
            sum += (i + 1) as u32 * hand.winnings;
        }

        assert_eq!(sum, 6440);
    }
}
