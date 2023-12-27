use crate::input;

pub fn solve() {
    let x = input::file_for_day(11);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x, 1_000_000));
}

fn part_one(input: Vec<String>) -> i64 {
    expand_and_check_distance(&input, 2)
}

fn part_two(input: Vec<String>, factor: usize) -> i64 {
    expand_and_check_distance(&input, factor)
}

fn expand_and_check_distance(input: &[String], factor: usize) -> i64 {
    let grid = input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut rows_to_expand = vec![true; grid.len()];
    let mut cols_to_expand = vec![true; grid[0].len()];

    for (i, x) in grid.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            let is_empty = *y == '.';
            rows_to_expand[i] = rows_to_expand[i] && is_empty;
            cols_to_expand[j] = cols_to_expand[j] && is_empty;
        }
    }

    let mut nodes = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if v == '#' {
                let node = crate::path::Node(i, j);
                nodes.push(node);
            }
        }
    }

    let node_pairs = nodes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| nodes[i + 1..].iter().map(move |b| (a, b)))
        .collect::<Vec<_>>();

    let adjusted_factor = factor - 1;

    node_pairs
        .iter()
        .map(|(&n1, &n2)| {
            let (i1, i2) = if n2.0 > n1.0 {
                (n1.0, n2.0)
            } else {
                (n2.0, n1.0)
            };

            let (j1, j2) = if n2.1 > n1.1 {
                (n1.1, n2.1)
            } else {
                (n2.1, n1.1)
            };

            let rows_to_expand: usize = rows_to_expand[i1..i2].iter().map(|&x| x as usize).sum();
            let cols_to_expand: usize = cols_to_expand[j1..j2].iter().map(|&x| x as usize).sum();

            let (n1i, n2i) = if n1.0 > n2.0 {
                (n1.0 + rows_to_expand * adjusted_factor, n2.0)
            } else {
                (n1.0, n2.0 + rows_to_expand * adjusted_factor)
            };

            let (n1j, n2j) = if n1.1 > n2.1 {
                (n1.1 + cols_to_expand * adjusted_factor, n2.1)
            } else {
                (n1.1, n2.1 + cols_to_expand * adjusted_factor)
            };

            let nn1 = crate::path::Node(n1i, n1j);
            let nn2 = crate::path::Node(n2i, n2j);

            crate::path::manhattan_distance(nn1, nn2)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 374;
    static SOLUTION_TWO: i64 = 8410;
    static TEST_INPUT: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_two(x, 100), SOLUTION_TWO);
    }
}
