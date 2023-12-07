use std::{collections::HashMap, str::FromStr};

const STENGTHS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

type RawHand = [char; 5];

#[derive(thiserror::Error, Debug)]
pub enum ParseHandErr {
    #[error("unknown hand format")]
    UnknownFormat,

    #[error("failed to parse the bid amount")]
    BadBidFormat,

    #[error("failed to parse the hand")]
    BadHandFormat,

    #[error("unknown card value: {0}")]
    UnknownCard(char),
}

#[derive(Debug, Clone, Eq)]
pub struct Hand {
    hand: RawHand,
    ty: HandType,
    pub bid: u32,
    jokers: bool,
}

impl Hand {
    fn new(hand: RawHand, ty: HandType, bid: u32) -> Self {
        Self {
            hand,
            ty,
            bid,
            jokers: false,
        }
    }

    /// Enable jokers & recalculate the hand's type
    pub fn enable_joker_cards(&mut self) {
        self.ty = HandType::from_raw(&self.hand, true);
        self.jokers = true;
    }
}

impl FromStr for Hand {
    type Err = ParseHandErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().split_ascii_whitespace().collect::<Vec<_>>();
        if s.len() != 2 {
            return Err(ParseHandErr::UnknownFormat);
        }

        let bid = s[1].parse().map_err(|_| ParseHandErr::BadBidFormat)?;
        let hand: RawHand = s[0]
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ParseHandErr::BadHandFormat)?;

        if let Some(ch) = hand.iter().find(|ch| !STENGTHS.contains(ch)) {
            // the hand contains an unknwon card
            return Err(ParseHandErr::UnknownCard(*ch));
        }

        Ok(Self::new(hand, HandType::from_raw(&hand, false), bid))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn hand_into_values(hand: &RawHand, jokers: bool) -> Vec<usize> {
            hand.iter()
                .map(|ch| {
                    if jokers && *ch == 'J' {
                        0
                    } else {
                        STENGTHS.iter().position(|card| card == ch).unwrap() + 1
                    }
                })
                .collect()
        }

        self.ty.cmp(&other.ty).then(
            hand_into_values(&self.hand, self.jokers)
                .cmp(&hand_into_values(&other.hand, self.jokers)),
        )
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_raw(hand: &RawHand, jokers: bool) -> Self {
        // map the hand into a list of counts of each label, sorted
        let mut map: HashMap<&char, usize> = HashMap::new();
        for ch in hand {
            *map.entry(ch).or_default() += 1;
        }
        let mut extras = 0usize;
        if jokers && map.len() > 1 {
            extras = map.remove(&'J').unwrap_or(0);
        }

        let mut hand = map.into_values().collect::<Vec<_>>();
        hand.sort();
        hand.reverse();
        hand[0] += extras; // increase the best label

        // map the type
        match hand.len() {
            1 => Self::FiveOfAKind,
            2 => match (hand[0], hand[1]) {
                (4, 1) => Self::FourOfAKind,
                (3, 2) => Self::FullHouse,
                _ => unreachable!(),
            },
            3 => match (hand[0], hand[1]) {
                (3, _) => Self::ThreeOfAKind,
                (2, 2) => Self::TwoPair,
                _ => unreachable!(),
            },
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Hand, HandType, RawHand};

    #[test]
    fn parse_hand_type() {
        let inputs = [
            "AAAAA", "AA8AA", "23332", "TTT98", "23432", "A23A4", "23456",
        ]
        .into_iter()
        .map(|hand| -> RawHand { hand.chars().collect::<Vec<_>>().try_into().unwrap() });
        let expected_outputs = [
            HandType::FiveOfAKind,
            HandType::FourOfAKind,
            HandType::FullHouse,
            HandType::ThreeOfAKind,
            HandType::TwoPair,
            HandType::OnePair,
            HandType::HighCard,
        ];

        for (input, expected) in inputs.into_iter().zip(expected_outputs) {
            let output = HandType::from_raw(&input, false);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn parse_hand() {
        let input = "32T3K 765";
        let expected_output = Hand {
            bid: 765,
            hand: ['3', '2', 'T', '3', 'K'],
            ty: HandType::OnePair,
            jokers: false,
        };

        let output: Hand = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn cmp_hand_with_same_type() {
        let hand1: Hand = "KK677 0".parse().unwrap();
        let hand2: Hand = "KTJJT 0".parse().unwrap();

        assert_eq!(hand1.ty, hand2.ty);
        assert!(hand1 > hand2);
    }

    #[test]
    fn cmp_hand_with_different_types() {
        let hand1: Hand = "KK677 0".parse().unwrap();
        let hand2: Hand = "32T3K 0".parse().unwrap();

        assert_ne!(hand1.ty, hand2.ty);
        assert!(hand1 > hand2);
    }

    #[test]
    fn enable_joker_cards() {
        let mut hand: Hand = "QJJQ2 0".parse().unwrap();
        assert_eq!(hand.ty, HandType::TwoPair);

        hand.enable_joker_cards();
        assert_eq!(hand.ty, HandType::FourOfAKind);
    }
}
