use crate::ReturnSize;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HandType::FiveOfKind => "FiveOfKind",
                HandType::FourOfKind => "FourOfKind",
                HandType::FullHouse => "FullHouse",
                HandType::ThreeOfKind => "ThreeOfKind",
                HandType::TwoPair => "TwoPair",
                HandType::OnePair => "OnePair",
                HandType::HighCard => "HighCard",
            }
        )
    }
}

#[derive(PartialEq, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    hand_type: HandType,
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.cards.iter().collect::<String>(),
            self.hand_type,
            self.bid
        )
    }
}

impl Hand {
    fn card_rank(card: char) -> u8 {
        match card {
            'W' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0, // unknown card
        }
    }

    fn compare(&self, other: &Hand) -> Ordering {
        let hand_type_order = self.hand_type.cmp(&other.hand_type);
        if hand_type_order != Ordering::Equal {
            return hand_type_order;
        }

        self.cards
            .iter()
            .zip(other.cards.iter())
            .map(|(&self_card, &other_card)| {
                // reverse due to the stronger hand coming first, instead of "smaller bigger" order
                Hand::card_rank(other_card).cmp(&Hand::card_rank(self_card))
            })
            .find(|&ordering| ordering != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }

    fn get_type(cards: String, jokers: bool) -> HandType {
        let mut counts = HashMap::new();
        let mut joker_count = 0;

        for card in cards.chars() {
            // check if wildcard
            if jokers && card == 'W' {
                joker_count += 1;
            } else {
                *counts.entry(card).or_insert(0) += 1;
            }
        }

        // If jokers are present, try to form the strongest hand
        if jokers {
            let max_count = counts.values().max().cloned().unwrap_or(0);

            // Adjust the counts based on the number of jokers
            let new_count = max_count + joker_count;

            match (new_count, joker_count) {
                (5, _) => return HandType::FiveOfKind,
                (4, _) => return HandType::FourOfKind,
                (3, 2) | (3, 1) | (3, _) if counts.len() == 2 => return HandType::FullHouse,
                (3, _) => return HandType::ThreeOfKind,
                (2, 2) | (2, 1) | (2, _) if counts.len() == 3 => return HandType::TwoPair,
                (2, _) => return HandType::OnePair,
                _ => (),
            }
        }

        let mut counts: Vec<_> = counts.into_iter().map(|(_card, count)| count).collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));

        match counts.as_slice() {
            [5] => HandType::FiveOfKind,
            [4, ..] => HandType::FourOfKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn sort_weak2strong(hands: &mut Vec<Hand>) {
        // reverse; greater value hand comes first
        hands.sort_by(|a, b| b.compare(a));
    }

    fn new(line: &str, jokers: bool) -> Self {
        let (cards, bid) = line.split_once(" ").expect("Failed to parse hand");
        let mut cards = cards.to_string();

        // rewrite jokers to wildcard mapping if enabled
        if jokers {
            cards = cards.replace("J", "W");
        }

        Self {
            cards: cards.chars().collect(),
            hand_type: Hand::get_type(cards, jokers),
            bid: bid.parse().expect("Failed to parse bid"),
        }
    }
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .split("\n")
        .map(|line| Hand::new(line, false))
        .collect();
    Hand::sort_weak2strong(&mut hands);

    return hands
        .iter()
        .enumerate()
        .map(|(place, hand)| hand.bid * (place + 1))
        .sum();
}

fn part2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .split("\n")
        .map(|line| Hand::new(line, true))
        .collect();
    Hand::sort_weak2strong(&mut hands);

    return hands
        .iter()
        .enumerate()
        .map(|(place, hand)| hand.bid * (place + 1))
        .sum();
}

pub fn solve_day() -> ReturnSize {
    let input = include_str!("../inputs/day7");

    return ReturnSize::USIZE((part1(&input), part2(&input)));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, Hand};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_test() {
        assert_eq!(6440, part1(&INPUT));
    }

    #[test]
    fn part2_test() {
        assert_eq!(5905, part2(&INPUT));
    }

    #[test]
    fn wildcard_sort_test() {
        let h1 = Hand::new("JKKK2 1", true);
        let h2 = Hand::new("QQQQ2 1", true);
        let mut hands = vec![h2, h1]; // should sort out
        Hand::sort_weak2strong(&mut hands);
        assert_eq!(
            vec![Hand::new("JKKK2 1", true), Hand::new("QQQQ2 1", true)],
            hands
        );
    }
}
