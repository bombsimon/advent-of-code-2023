use crate::input;

pub fn solve() {
    let x = input::file_for_day(6);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let (times, distances) = times_and_distances(input.as_ref(), false);

    times
        .into_iter()
        .zip(distances)
        .map(find_possible_lengths)
        .product()
}

fn part_two(input: Vec<String>) -> i64 {
    let (times, distances) = times_and_distances(input.as_ref(), true);
    let time = *times.first().unwrap();
    let distance_to_beat = *distances.first().unwrap();

    find_possible_lengths((time, distance_to_beat))
}

fn times_and_distances(input: &[String], single_digit: bool) -> (Vec<f64>, Vec<f64>) {
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
        .map(|t| t.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let distances = l2
        .split_whitespace()
        .map(|t| t.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    (times, distances)
}

fn find_possible_lengths((race_duration, distance_to_beat): (f64, f64)) -> i64 {
    let a = 1f64;
    let b = -race_duration;
    let c = distance_to_beat;

    let discriminant = b.powf(2.) - 4. * a * c;
    if discriminant < 0. {
        return 0;
    }

    let sqrt_discriminant = discriminant.sqrt();
    let root1 = (-b + sqrt_discriminant) / (2. * a);
    let root2 = (-b - sqrt_discriminant) / (2. * a);

    (root1.ceil() - root2.floor()) as i64 - 1
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
