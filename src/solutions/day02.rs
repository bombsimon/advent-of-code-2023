use crate::input;

pub fn solve() {
    let x = input::file_for_day(2);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

#[derive(Debug)]
struct Set {
    r: i64,
    g: i64,
    b: i64,
}

#[derive(Debug)]
struct Game {
    id: i64,
    sets: Vec<Set>,
}

impl Game {
    fn new(input: &str) -> Self {
        let line = input.replace("Game ", "");
        let mut parts = line.split(": ");
        let id = parts.next().unwrap().parse::<i64>().unwrap();
        let sets = parts.next().unwrap().split("; ");

        let mut game = Game {
            id,
            sets: Default::default(),
        };

        for set in sets {
            let mut s = Set { r: 0, g: 0, b: 0 };

            let cubes = set.split(", ");
            for cube in cubes {
                let mut n_color = cube.split_whitespace();
                let n = n_color.next().unwrap().parse::<i64>().unwrap();
                match n_color.next().unwrap() {
                    "red" => s.r = n,
                    "green" => s.g = n,
                    "blue" => s.b = n,
                    _ => unreachable!(),
                }
            }

            game.sets.push(s);
        }

        game
    }

    fn min_cubes_pow(&self) -> i64 {
        let mut rgb = (0, 0, 0);
        for set in &self.sets {
            rgb = (rgb.0.max(set.r), rgb.1.max(set.g), rgb.2.max(set.b));
        }

        rgb.0 * rgb.1 * rgb.2
    }

    fn valid_game(&self) -> bool {
        !self
            .sets
            .iter()
            .any(|set| set.r > 12 || set.g > 13 || set.b > 14)
    }
}

fn part_one(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|l| Game::new(l))
        .map(|g| if g.valid_game() { g.id } else { 0 })
        .sum()
}

fn part_two(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|l| Game::new(l))
        .map(|g| g.min_cubes_pow())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 8;
    static SOLUTION_TWO: i64 = 2286;
    static TEST_INPUT: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
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
