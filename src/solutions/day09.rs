use crate::input;

pub fn solve() {
    let x = input::file_for_day(9);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let sequences = input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut last_values = Vec::new();
    for sequence in sequences {
        let last = sequence.last().unwrap().to_owned();
        let mut current_sequence_end = sequence;
        let mut last_digits = Vec::new();

        loop {
            current_sequence_end = current_sequence_end
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>();

            if current_sequence_end.iter().all(|&n| n == 0) {
                let next = last + last_digits.iter().sum::<i64>();
                last_values.push(next);

                break;
            }

            last_digits.push(current_sequence_end[current_sequence_end.len() - 1]);
        }
    }

    last_values.iter().sum()
}

fn part_two(input: Vec<String>) -> i64 {
    let sequences = input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut first_values = Vec::new();
    for sequence in sequences {
        let first = sequence.first().unwrap().to_owned();
        let mut current_sequence_end = sequence;
        let mut first_digits = Vec::new();

        loop {
            current_sequence_end = current_sequence_end
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>();

            if current_sequence_end.iter().all(|&n| n == 0) {
                first_digits.reverse();
                let previous = first - first_digits.iter().fold(0, |acc, n| n - acc);
                first_values.push(previous);

                break;
            }

            first_digits.push(current_sequence_end[0]);
        }
    }

    first_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 114;
    static SOLUTION_TWO: i64 = 2;
    static TEST_INPUT: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

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
