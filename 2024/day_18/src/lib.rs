use std::collections::{HashSet, VecDeque};

pub mod part_1;
pub mod part_2;

pub fn get_minimum_steps(
    bytes: &[(usize, usize)],
    grid_size: usize,
    bytes_fallen: usize,
) -> Option<usize> {
    let mut memory_space = vec![vec![Memory::Safe; grid_size]; grid_size];

    for (x, y) in bytes.iter().take(bytes_fallen) {
        memory_space[*x][*y] = Memory::Corrupted;
    }

    bfs(&memory_space, (0, 0), (grid_size - 1, grid_size - 1))
}

fn bfs(grid: &[Vec<Memory>], start: (usize, usize), finish: (usize, usize)) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(((x, y), dist)) = queue.pop_front() {
        if (x, y) == finish {
            return Some(dist);
        }

        for &(dx, dy) in &directions {
            if let Some((nx, ny)) = valid_neighbour(grid.len(), x, dx, y, dy) {
                if grid[nx][ny] != Memory::Corrupted && !visited.contains(&(nx, ny)) {
                    visited.insert((nx, ny));
                    queue.push_back(((nx, ny), dist + 1));
                }
            }
        }
    }

    None
}

fn valid_neighbour(
    size: usize,
    x: usize,
    dx: isize,
    y: usize,
    dy: isize,
) -> Option<(usize, usize)> {
    let x = x.checked_add_signed(dx)?;
    let y = y.checked_add_signed(dy)?;

    if x < size && y < size {
        Some((x, y))
    } else {
        None
    }
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let mut numbers = l.split(',');

            let mut p = || numbers.next().expect("Number").parse().expect("Valid");

            (p(), p())
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Memory {
    Safe,
    Corrupted,
}
