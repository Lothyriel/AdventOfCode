use crate::{parse, Matrix, Point};

pub fn hoof_it(input: &str) -> usize {
    let grid = parse(input);

    grid.get_coordinates(|x| *x == 0)
        .map(|head| dfs(&grid, head))
        .sum()
}

fn dfs(grid: &Matrix<usize>, (pos, current): (Point, usize)) -> usize {
    if current == 9 {
        return 1;
    }

    let mut paths = 0;

    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new = (pos.0 as isize + dx, pos.1 as isize + dy);

        if let Some(new) = grid.valid_coordinates(new) {
            let new_value = grid.get(new).expect("Valid");
            if new_value == current + 1 {
                paths += dfs(grid, (new, new_value));
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        let result = hoof_it(input);
        assert_eq!(81, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = hoof_it(input);
        assert_eq!(1805, result);
    }
}
