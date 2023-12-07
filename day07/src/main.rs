mod part1 {
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Card {
        N2,
        N3,
        N4,
        N5,
        N6,
        N7,
        N8,
        N9,
        T,
        J,
        Q,
        K,
        A,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Hand([Card; 5]);

    impl From<&str> for Hand {
        fn from(value: &str) -> Self {
            let mut cards = [Card::A; 5];
            for (in_val, out_card) in value.chars().zip(cards.iter_mut()) {
                *out_card = match in_val {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
                    'T' => Card::T,
                    '9' => Card::N9,
                    '8' => Card::N8,
                    '7' => Card::N7,
                    '6' => Card::N6,
                    '5' => Card::N5,
                    '4' => Card::N4,
                    '3' => Card::N3,
                    '2' => Card::N2,

                    _ => panic!("Invalid card: {in_val}"),
                };
            }
            Self(cards)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        // High card, where all cards' labels are distinct: 23456
        HighCard(Hand),
        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        OnePair(Hand),
        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        TwoPair(Hand),
        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        ThreeOfKind(Hand),
        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        FullHouse(Hand),
        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        FourOfKind(Hand),
        // Five of a kind, where all five cards have the same label: AAAAA
        FiveOfKind(Hand),
    }

    impl From<Hand> for HandType {
        fn from(value: Hand) -> Self {
            let mut card_counts: HashMap<Card, u32> = HashMap::new();
            for card in value.0 {
                *card_counts.entry(card).or_default() += 1;
            }
            let mut card_counts: Vec<u32> = card_counts.into_values().collect();
            card_counts.sort();
            card_counts.reverse();
            if card_counts[0] == 5 {
                HandType::FiveOfKind(value)
            } else if card_counts[0] == 4 {
                HandType::FourOfKind(value)
            } else if card_counts[0] == 3 && card_counts[1] == 2 {
                HandType::FullHouse(value)
            } else if card_counts[0] == 3 {
                HandType::ThreeOfKind(value)
            } else if card_counts[0] == 2 && card_counts[1] == 2 {
                HandType::TwoPair(value)
            } else if card_counts[0] == 2 {
                HandType::OnePair(value)
            } else {
                HandType::HighCard(value)
            }
        }
    }

    struct HandBid {
        hand: HandType,
        bid: u32,
    }

    impl From<&str> for HandBid {
        fn from(value: &str) -> Self {
            let (hand, bid) = value.trim().split_once(' ').unwrap();
            let hand: Hand = hand.into();
            let hand_type = hand.into();
            let bid = bid.parse().unwrap();

            Self {
                hand: hand_type,
                bid,
            }
        }
    }

    fn parse(input: &[&str]) -> Vec<HandBid> {
        let mut hand_bids = Vec::new();
        for line in input.iter() {
            hand_bids.push((*line).into());
        }

        hand_bids
    }

    pub fn solve1(input: &[&str]) -> u32 {
        let mut hand_bids = parse(input);
        hand_bids.sort_unstable_by_key(|h| h.hand);
        let mut total = 0;

        for (rank, hand_bid) in hand_bids.into_iter().enumerate() {
            total += (rank as u32 + 1) * hand_bid.bid;
        }
        total
    }
}

mod part2 {
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Card {
        J,
        N2,
        N3,
        N4,
        N5,
        N6,
        N7,
        N8,
        N9,
        T,
        Q,
        K,
        A,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Hand([Card; 5]);

    impl From<&str> for Hand {
        fn from(value: &str) -> Self {
            let mut cards = [Card::A; 5];
            for (in_val, out_card) in value.chars().zip(cards.iter_mut()) {
                *out_card = match in_val {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
                    'T' => Card::T,
                    '9' => Card::N9,
                    '8' => Card::N8,
                    '7' => Card::N7,
                    '6' => Card::N6,
                    '5' => Card::N5,
                    '4' => Card::N4,
                    '3' => Card::N3,
                    '2' => Card::N2,

                    _ => panic!("Invalid card: {in_val}"),
                };
            }
            Self(cards)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        // High card, where all cards' labels are distinct: 23456
        HighCard(Hand),
        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        OnePair(Hand),
        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        TwoPair(Hand),
        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        ThreeOfKind(Hand),
        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        FullHouse(Hand),
        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        FourOfKind(Hand),
        // Five of a kind, where all five cards have the same label: AAAAA
        FiveOfKind(Hand),
    }

    impl From<Hand> for HandType {
        fn from(value: Hand) -> Self {
            let mut card_counts: HashMap<Card, u32> = HashMap::new();
            let mut num_jokers = 0;
            for card in value.0 {
                if let Card::J = card {
                    num_jokers += 1;
                } else {
                    *card_counts.entry(card).or_default() += 1;
                }
            }
            let mut card_counts: Vec<u32> = card_counts.into_values().collect();
            
            // special case, in case hand all jokers
            card_counts.push(0);

            card_counts.sort();
            card_counts.reverse();
            if card_counts[0] + num_jokers == 5 {
                HandType::FiveOfKind(value)
            } else if card_counts[0] + num_jokers == 4 {
                HandType::FourOfKind(value)
            } else if (card_counts[0] == 3 && card_counts[1] == 2) ||
            (card_counts[0] == 2 && card_counts[1] == 2 && num_jokers == 1) {
                HandType::FullHouse(value)
            } else if card_counts[0] + num_jokers == 3 {
                HandType::ThreeOfKind(value)
            } else if card_counts[0] == 2 && card_counts[1] == 2 {
                HandType::TwoPair(value)
            } else if card_counts[0] + num_jokers == 2 {
                HandType::OnePair(value)
            } else {
                HandType::HighCard(value)
            }
        }
    }

    struct HandBid {
        hand: HandType,
        bid: u32,
    }

    impl From<&str> for HandBid {
        fn from(value: &str) -> Self {
            let (hand, bid) = value.trim().split_once(' ').unwrap();
            let hand: Hand = hand.into();
            let hand_type = hand.into();
            let bid = bid.parse().unwrap();

            Self {
                hand: hand_type,
                bid,
            }
        }
    }

    fn parse(input: &[&str]) -> Vec<HandBid> {
        let mut hand_bids = Vec::new();
        for line in input.iter() {
            hand_bids.push((*line).into());
        }

        hand_bids
    }

    pub fn solve2(input: &[&str]) -> u32 {
        let mut hand_bids = parse(input);
        hand_bids.sort_unstable_by_key(|h| h.hand);
        let mut total = 0;

        for (rank, hand_bid) in hand_bids.into_iter().enumerate() {
            total += (rank as u32 + 1) * hand_bid.bid;
        }
        total
    }
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", part1::solve1(&input));
    println!("part 2: {}", part2::solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];

    #[test]
    fn test1() {
        assert_eq!(part1::solve1(INPUT), 6440)
    }

    #[test]
    fn test2() {
        assert_eq!(part2::solve2(INPUT), 5905)
    }
}
