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
    input
        .iter_mut()
        .map(|line| {
            let mut low = (None, None, 0);
            let mut high = (None, None, 0);

            let digits = vec![
                ("one", "1"),
                ("two", "2"),
                ("three", "3"),
                ("four", "4"),
                ("five", "5"),
                ("six", "6"),
                ("seven", "7"),
                ("eight", "8"),
                ("nine", "9"),
            ];

            for digit in digits.iter() {
                if let Some(n) = line.find(digit.0) {
                    if n < low.2 || low.0.is_none() {
                        low = (Some(digit.0), Some(digit.1), n);
                    }
                }

                if let Some(n) = line.rfind(digit.0) {
                    if n > high.2 || high.0.is_none() {
                        high = (Some(digit.0), Some(digit.1), n);
                    }
                }
            }

            if low.0.is_some() {
                line.insert_str(low.2, low.1.unwrap());
            }

            if high.0.is_some() {
                let extra = if low.0.is_none() { 0 } else { 1 };
                line.insert_str(high.2 + extra, high.1.unwrap());
            }

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
