use crate::input;

pub fn solve() {
    let x = input::file_for_day(1);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|line| {
            let n = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();

            let first = n.first().unwrap();
            let last = n.last().unwrap();

            format!("{}{}", first, last).parse::<i64>().unwrap()
        })
        .sum()
}

fn part_two(mut input: Vec<String>) -> i64 {
    let patterns: [(&str, i64); 18] = [
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ];

    input
        .iter_mut()
        .map(|line| {
            let left = patterns
                .into_iter()
                .fold(
                    (usize::MAX, i64::MAX),
                    |(left_most_idx, left_most_value), (pattern, value)| match line.find(pattern) {
                        Some(n) if n <= left_most_idx => (n, value * 10),
                        _ => (left_most_idx, left_most_value),
                    },
                )
                .1;

            let right = patterns
                .into_iter()
                .fold(
                    (0, 0),
                    |(right_most_idx, right_most_value), (pattern, value)| match line.rfind(pattern)
                    {
                        Some(n) if n >= right_most_idx => (n, value),
                        _ => (right_most_idx, right_most_value),
                    },
                )
                .1;

            left + right
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 142;
    static SOLUTION_TWO: i64 = 281;
    static TEST_INPUT: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
    static TEST_INPUT_2: &str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
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
