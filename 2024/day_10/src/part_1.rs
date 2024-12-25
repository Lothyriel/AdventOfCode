use std::collections::HashSet;

use crate::{parse, Matrix, Point};

pub fn hoof_it(input: &str) -> usize {
    let grid = parse(input);

    grid.get_coordinates(|x| *x == 0)
        .map(|head| dfs(&grid, head, &mut HashSet::new()))
        .sum()
}

fn dfs(
    grid: &Matrix<usize>,
    (pos, current): (Point, usize),
    visited: &mut HashSet<Point>,
) -> usize {
    visited.insert(pos);
    let mut score = 0;

    if current == 9 {
        return 1;
    }

    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new = (pos.0 as isize + dx, pos.1 as isize + dy);

        if let Some(new) = grid.valid_coordinates(new) {
            let new_value = grid.get(new).expect("Valid");
            if !visited.contains(&new) && new_value == current + 1 {
                score += dfs(grid, (new, new_value), visited);
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r#"0123
1234
8765
9876"#;

        let result = hoof_it(input);
        assert_eq!(1, result);
    }

    #[test]
    fn example_2() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        let result = hoof_it(input);
        assert_eq!(36, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = hoof_it(input);
        assert_eq!(825, result);
    }
}
