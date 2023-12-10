use std::io::Write;

use crate::{
    input::{self},
    path::{Graph, Node},
};

pub fn solve() {
    let x = input::file_for_day(10);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let grid = input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (graph, start_node) = build_graph(&grid);

    graph.bfs(start_node).len() as i64 / 2
}

fn part_two(input: Vec<String>) -> i64 {
    let grid = input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (graph, start_node) = build_graph(&grid);
    let result = graph.bfs(start_node);

    let mut enclosed = 0;

    for (i, _) in grid.iter().enumerate() {
        // Angles in this input consists of two characters (L7 or FJ) so we keep track of seen
        // angles and if we see the pairing character to make a full angle we've passed a wall
        // similar to a `|`.
        // If we see a wall in the opposite direction we reset the state and assume it must be a
        // dead pipe.
        let mut angles = [false; 2];
        let mut walls = 0;

        for j in 0..grid[i].len() {
            if !result.contains(&Node(i, j)) && walls % 2 == 1 {
                enclosed += 1;
                continue;
            }

            if result.contains(&Node(i, j)) {
                match grid[i][j] {
                    '|' => walls += 1,
                    'L' => {
                        angles[0] = true;
                        angles[1] = false;
                    }
                    '7' if angles[0] => {
                        walls += 1;
                        angles[0] = false;
                    }
                    'F' => {
                        angles[1] = true;
                        angles[0] = false;
                    }
                    'J' if angles[1] => {
                        walls += 1;
                        angles[1] = false;
                    }
                    _ => (),
                };
            }
        }
    }

    enclosed - 1
}

fn build_graph(grid: &[Vec<char>]) -> (Graph, Node) {
    let height = grid.len();
    let width = grid[0].len();

    let mut graph = Graph::new();
    let mut start_node = Node(0, 0);

    for i in 0..height {
        for j in 0..width {
            let current_node = Node(i, j);
            graph.add_node(current_node);

            let current_pipe = grid[i][j];
            for (direction, i1, j1) in [('D', i + 1, j), ('R', i, j + 1)] {
                if i1 >= height || j1 >= width {
                    continue;
                }

                let neighbour_node = Node(i1, j1);
                let adjecent_pipe = grid[i1][j1];

                if current_pipe == 'S' {
                    start_node = current_node;
                }

                match (direction, current_pipe, adjecent_pipe) {
                    ('D', 'S' | '|' | 'F' | '7', '|' | 'J' | 'L') => {
                        graph.add_node(neighbour_node);
                        graph.add_edge(current_node, neighbour_node);
                    }
                    ('R', 'S' | '-', '-' | 'J' | '7') => {
                        graph.add_node(neighbour_node);
                        graph.add_edge(current_node, neighbour_node);
                    }
                    ('R', 'F' | 'L', '-' | '7' | 'J') => {
                        graph.add_node(neighbour_node);
                        graph.add_edge(current_node, neighbour_node);
                    }
                    _ => (),
                }
            }
        }
    }

    (graph, start_node)
}

#[allow(dead_code)]
fn print_and_mark(grid: &[Vec<char>], nodes: &[Node]) {
    for (i, _) in grid.iter().enumerate() {
        for j in 0..grid[i].len() {
            let c = if nodes.contains(&Node(i, j)) {
                let p = match grid[i][j] {
                    '-' => '═',
                    '7' => '╗',
                    'F' => '╔',
                    'J' => '╝',
                    'L' => '╚',
                    '|' => '║',
                    a => a,
                };

                format!("{}", p)
            } else {
                " ".to_string()
            };

            print!("{c}");
        }

        println!();
    }

    std::io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 8;
    static SOLUTION_TWO: i64 = 10;
    static TEST_INPUT: &str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    static TEST_INPUT_TWO: &str = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

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
