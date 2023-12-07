use itertools::Itertools;

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Joker = 1,
    JesTer = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::JesTer,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid input character {}", c),
        }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandKind {
    fn from_hand(hand: &[Card]) -> Self {
        let mut counts: Vec<usize> = hand
            .iter()
            .filter(|c| c != &&Card::Joker)
            .counts()
            .values()
            .sorted()
            .rev()
            .copied()
            .collect();

        let jokers = hand.iter().filter(|c| c == &&Card::Joker).count();

        if counts.is_empty() {
            counts.push(0)
        }
        counts[0] += jokers;

        if counts[0] == 5 {
            HandKind::FiveOfAKind
        } else if counts[0] == 4 {
            HandKind::FourOfAKind
        } else if counts[0] == 3 && counts[1] == 2 {
            HandKind::FullHouse
        } else if counts[0] == 3 {
            HandKind::ThreeOfAKind
        } else if counts[1] == 2 {
            HandKind::TwoPair
        } else if counts[0] == 2 {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    bid: u32,
    kind: HandKind,
    hand: Vec<Card>,
}

impl Hand {
    fn from_string(input: &str) -> Self {
        let mut input = input.split_whitespace();
        let hand = input
            .next()
            .unwrap()
            .chars()
            .map(Card::from_char)
            .collect::<Vec<_>>();

        let kind = HandKind::from_hand(&hand);

        let bid = input.next().unwrap().parse().unwrap();
        Hand { kind, hand, bid }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        for (i, card) in self.hand.iter().enumerate() {
            match card.partial_cmp(&other.hand[i]) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }
        Some(core::cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(Hand::from_string)
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = include_str!("../test_files/part_one_and_two.txt");
        assert_eq!(part_two(input), 5905)
    }
}
