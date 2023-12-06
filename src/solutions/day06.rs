use crate::input;

pub fn solve() {
    let x = input::file_for_day(6);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    win_product(input.as_ref(), false)
}

fn part_two(input: Vec<String>) -> i64 {
    win_product(input.as_ref(), true)
}

fn win_product(input: &[String], single_digit: bool) -> i64 {
    let (times, distances) = times_and_distances(input, single_digit);

    times
        .iter()
        .enumerate()
        .map(|(i, time)| {
            let mut wins = 0;
            let to_beat = distances[i];

            let mut previous = 0;
            for n in 0..*time {
                let speed = n;
                let time_for_speed = time - n;
                let distance_travelled = speed * time_for_speed;

                if distance_travelled > to_beat {
                    wins += 1;
                } else if distance_travelled < previous {
                    // We've peaked and don't have a winning distance so abort.
                    break;
                }

                previous = distance_travelled;
            }

            wins
        })
        .product()
}

fn times_and_distances(input: &[String], single_digit: bool) -> (Vec<i64>, Vec<i64>) {
    let replace_space = if single_digit { "" } else { " " };
    let l1 = input
        .first()
        .unwrap()
        .replace("Time:", "")
        .replace(' ', replace_space);

    let l2 = input
        .last()
        .unwrap()
        .replace("Distance:", "")
        .replace(' ', replace_space);

    let times = l1
        .split_whitespace()
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let distances = l2
        .split_whitespace()
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    (times, distances)
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 288;
    static SOLUTION_TWO: i64 = 71503;
    static TEST_INPUT: &str = r#"
Time:      7  15   30
Distance:  9  40  200
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
