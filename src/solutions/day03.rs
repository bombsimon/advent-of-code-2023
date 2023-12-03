use crate::input;

pub fn solve() {
    let x = input::file_for_day(3);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let grid = input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut numbers_with_adjecent = Vec::new();

    for i in 0..grid.len() {
        let mut current_number = Vec::new();
        let mut current_number_has_match = false;

        for j in 0..grid[i].len() {
            let col = grid[i][j];
            let is_digit = col.is_ascii_digit();

            if !is_digit {
                if current_number_has_match {
                    numbers_with_adjecent.push(
                        current_number
                            .iter()
                            .collect::<String>()
                            .parse::<i64>()
                            .unwrap(),
                    );
                }

                current_number.clear();
                current_number_has_match = false;
                continue;
            }

            // Push this to our current number and just continue if we already found a match.
            current_number.push(col);
            if current_number_has_match {
                continue;
            }

            (current_number_has_match, _) = has_adjecent(&grid, (i, j));
        }

        // Don't forget to collect the number if it ended in the last column.
        if !current_number.is_empty() && current_number_has_match {
            numbers_with_adjecent.push(
                current_number
                    .iter()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap(),
            );
        }
    }

    numbers_with_adjecent.iter().sum()
}

fn part_two(_input: Vec<String>) -> i64 {
    0
}

fn has_adjecent(
    grid: &std::vec::Vec<std::vec::Vec<char>>,
    (i, j): (usize, usize),
) -> (bool, Option<(usize, usize)>) {
    let to_check: [(i32, i32); 8] = [
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, 0),
        (1, -1),
        (1, 1),
        (0, -1),
        (0, 1),
    ];

    for (i_add, j_add) in to_check {
        // Avoid overflow
        if (i == 0 && i_add < 0) || (j == 0 && j_add < 0) {
            continue;
        }

        // Convert to calculate new indices.
        let i1 = (i as i32 + i_add) as usize;
        let j1 = (j as i32 + j_add) as usize;

        if i1 >= grid.len() || j1 >= grid[i1].len() {
            continue;
        }

        match grid[i1][j1] {
            '.' => continue,
            n if n.is_ascii_digit() => continue,
            _ => return (true, Some((i1, j1))),
        }
    }

    (false, None)
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 4361;
    static SOLUTION_TWO: i64 = 0;
    static TEST_INPUT: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
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
