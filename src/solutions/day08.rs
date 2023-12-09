use std::collections::HashMap;

use crate::input;

pub fn solve() {
    let x = input::file_for_day(8);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    solve_for_pattern(&input, "AAA")
}

fn part_two(input: Vec<String>) -> i64 {
    solve_for_pattern(&input, "A")
}

fn solve_for_pattern(input: &[String], ends: &str) -> i64 {
    let (instructions, hm) = input_and_instructions(input);
    let mut nodes = hm.keys().filter(|k| k.ends_with(ends)).collect::<Vec<_>>();
    let mut steps_to_z = vec![0; nodes.len()];

    for (i, instruction) in instructions.iter().cycle().enumerate() {
        for j in 0..nodes.len() {
            if steps_to_z[j] != 0 {
                continue;
            }

            let node = nodes[j];
            let next_pair = hm.get(node).unwrap();

            nodes[j] = match instruction {
                'L' => &next_pair.0,
                'R' => &next_pair.1,
                _ => unreachable!(),
            };

            if nodes[j].ends_with('Z') {
                steps_to_z[j] = i + 1;
            }
        }

        if steps_to_z.iter().all(|n| *n != 0) {
            break;
        }
    }

    lcm(steps_to_z.as_ref()) as i64
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd_of_two_numbers(b, a % b)
}

fn input_and_instructions(input: &[String]) -> (Vec<char>, HashMap<String, (String, String)>) {
    let instructions = input.first().unwrap().chars().collect::<Vec<_>>();
    let hm = input
        .iter()
        .skip(2)
        .map(|row| {
            let mut p = row.split(" = ");
            let k = p.next().unwrap();

            let map = p.next().unwrap().replace(['(', ')'], "");
            let mut lr = map.split(", ");
            let left = lr.next().unwrap().to_owned();
            let right = lr.next().unwrap().to_owned();

            (k.to_owned(), (left, right))
        })
        .collect::<HashMap<String, (String, String)>>();

    (instructions, hm)
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 6;
    static SOLUTION_TWO: i64 = 6;
    static TEST_INPUT: &str = r#"
LLR
-
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    static TEST_INPUT_TWO: &str = r#"
LR
-
11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT_TWO);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
