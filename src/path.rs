#![allow(dead_code)]
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Node(pub usize, pub usize);

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Default::default(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.entry(node).or_default();
    }

    pub fn add_edge(&mut self, src: Node, dst: Node) {
        let n = self.nodes.get_mut(&src).unwrap();
        (*n).insert(dst);

        let e = self.nodes.get_mut(&dst).unwrap();
        (*e).insert(src);
    }

    pub fn contains(&self, node: Node) -> bool {
        self.nodes.contains_key(&node)
    }

    pub fn bfs(&self, start: Node) -> Vec<Node> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            result.push(current);

            let neightbours = self.nodes.get(&current).unwrap();
            for &neighbor in neightbours {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Filled,
    Wall,
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![vec![Cell::Empty; cols]; rows];
        Grid { cells }
    }

    pub fn add_wall(&mut self, row: usize, col: usize) {
        self.cells[row][col] = Cell::Wall;
    }

    pub fn cell_at(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    pub fn flood_fill(&mut self, start_row: usize, start_col: usize) {
        let mut visited = vec![vec![false; self.cells[0].len()]; self.cells.len()];
        let mut queue = VecDeque::new();

        queue.push_back((start_row, start_col));

        while let Some((row, col)) = queue.pop_front() {
            if row >= self.cells.len() || col >= self.cells[0].len() || visited[row][col] {
                continue;
            }

            visited[row][col] = true;

            if self.cells[row][col] == Cell::Wall {
                continue;
            }

            self.cells[row][col] = Cell::Filled;

            if row > 0 && !visited[row - 1][col] {
                queue.push_back((row - 1, col));
            }

            if row < self.cells.len() - 1 && !visited[row + 1][col] {
                queue.push_back((row + 1, col));
            }

            if col > 0 && !visited[row][col - 1] {
                queue.push_back((row, col - 1));
            }

            if col < self.cells[0].len() - 1 && !visited[row][col + 1] {
                queue.push_back((row, col + 1));
            }
        }
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Empty => write!(f, "  ")?,
                    Cell::Filled => write!(f, "░░")?,
                    Cell::Wall => write!(f, "▓▓")?,
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
