use crate::input;
use std::{cmp::Ordering, collections::HashMap};

pub fn solve() {
    let x = input::file_for_day(7);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let mut hands = input
        .iter()
        .map(|row| Hand::from_str(row, false))
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as i64)
        .sum()
}

fn part_two(input: Vec<String>) -> i64 {
    let mut hands = input
        .iter()
        .map(|row| Hand::from_str(row, true))
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as i64)
        .sum()
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bid: i64,
    pair: u8,
    triss: u8,
    four_of_a_kind: u8,
    five_of_a_kind: u8,
    wildcards: u8,
}

impl Hand {
    fn from_str(input: &str, with_jokers: bool) -> Self {
        let mut p = input.split_whitespace();
        let cards = p.next().unwrap().chars().collect::<Vec<_>>();
        let bid = p.next().unwrap().parse::<i64>().unwrap();

        let mut count = HashMap::new();
        cards.iter().for_each(|c| {
            *count.entry(c).or_insert(0) += 1;
        });

        let wildcards = if with_jokers {
            count.remove(&'J').unwrap_or(0)
        } else {
            0
        };

        let [mut pair, mut triss, mut four_of_a_kind, mut five_of_a_kind] = [0, 0, 0, 0];
        if wildcards == 5 {
            five_of_a_kind = 1;
        }

        let mut sorted_count = count.values().collect::<Vec<_>>();
        sorted_count.sort();
        sorted_count.reverse();

        for (i, &v) in sorted_count.into_iter().enumerate() {
            let value_with_joker = if i == 0 { v + wildcards } else { v };

            match value_with_joker {
                2 => pair += 1,
                3 => triss += 1,
                4 => four_of_a_kind += 1,
                5 => five_of_a_kind += 1,
                _ => (),
            }
        }

        Hand {
            cards,
            bid,
            pair,
            triss,
            four_of_a_kind,
            five_of_a_kind,
            wildcards,
        }
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (a, b) in [
            (self.five_of_a_kind, other.five_of_a_kind),
            (self.four_of_a_kind, other.four_of_a_kind),
            (self.triss, other.triss),
            (self.pair, other.pair),
        ] {
            if let Some(ordering) = greater_or_less(a, b) {
                return Some(ordering);
            }
        }

        for i in 0..self.cards.len() {
            let c1 = self.cards[i];
            let c2 = other.cards[i];

            if c1 == c2 {
                continue;
            }

            if let (Some(a), Some(b)) = (c1.to_digit(10), c2.to_digit(10)) {
                if let Some(ordering) = greater_or_less(a as u8, b as u8) {
                    return Some(ordering);
                }
            }

            return match (c1, c2) {
                ('J', _) if self.wildcards > 0 => Some(Ordering::Less),
                (_, 'J') if other.wildcards > 0 => Some(Ordering::Greater),

                (_, b) if b.is_ascii_digit() => Some(Ordering::Greater),
                (a, _) if a.is_ascii_digit() => Some(Ordering::Less),

                ('A', _) => Some(Ordering::Greater),
                ('K', b) if ['Q', 'J', 'T'].contains(&b) => Some(Ordering::Greater),
                ('K', _) => Some(Ordering::Less),
                ('Q', b) if ['J', 'T'].contains(&b) => Some(Ordering::Greater),
                ('Q', _) => Some(Ordering::Less),
                ('J', 'T') => Some(Ordering::Greater),
                ('J', _) => Some(Ordering::Less),
                ('T', _) => Some(Ordering::Less),

                (_, 'A') => Some(Ordering::Less),
                (a, 'K') if ['Q', 'J', 'T'].contains(&a) => Some(Ordering::Less),
                (_, 'K') => Some(Ordering::Greater),
                (a, 'Q') if ['J', 'T'].contains(&a) => Some(Ordering::Less),
                (_, 'Q') => Some(Ordering::Greater),
                (_, 'J') => Some(Ordering::Greater),
                (_, 'T') => Some(Ordering::Greater),

                (_, _) => unreachable!(),
            };
        }

        Some(Ordering::Equal)
    }
}

fn greater_or_less(a: u8, b: u8) -> Option<Ordering> {
    if a > b {
        return Some(Ordering::Greater);
    }

    if a < b {
        return Some(Ordering::Less);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 6440;
    static SOLUTION_TWO: i64 = 5905;
    static TEST_INPUT: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
