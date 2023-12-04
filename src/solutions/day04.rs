use std::collections::{HashMap, HashSet};

use crate::input;

pub fn solve() {
    let x = input::file_for_day(4);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    input
        .into_iter()
        .map(|l| {
            let line = l.replace("Card", "");
            let mut a = line.trim_start().split(": ");
            let _ = a.next().unwrap().parse::<i64>().unwrap();
            let mut p = a.next().unwrap().split(" | ");

            let lhs = p
                .next()
                .unwrap()
                .split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<HashSet<_>>();

            let rhs = p
                .next()
                .unwrap()
                .split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<HashSet<_>>();

            match rhs.intersection(&lhs).collect::<Vec<_>>().len() {
                0 => 0,
                len => 2u32.pow(len as u32 - 1) as i64,
            }
        })
        .sum()
}

fn part_two(input: Vec<String>) -> i64 {
    let mut scratchcards = HashMap::new();

    input.into_iter().for_each(|l| {
        let line = l.replace("Card", "");
        let mut a = line.trim_start().split(": ");
        let card_id = a.next().unwrap().parse::<usize>().unwrap();
        let mut p = a.next().unwrap().split(" | ");

        let lhs = p
            .next()
            .unwrap()
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();

        let rhs = p
            .next()
            .unwrap()
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();

        let intersections = rhs.intersection(&lhs).collect::<Vec<_>>().len();
        let cards_gotten = *scratchcards.entry(card_id).or_insert(1);

        for n in 0..intersections {
            let next_card = card_id + n + 1;
            *scratchcards.entry(next_card).or_insert(1) += cards_gotten;
        }
    });

    scratchcards.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 13;
    static SOLUTION_TWO: i64 = 30;
    static TEST_INPUT: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
    static TEST_INPUT_2: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT_2);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
