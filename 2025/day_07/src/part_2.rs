use std::collections::HashMap;

use lib::{Matrix, Point};

use crate::{Tile, parse};

pub fn laboratories(input: &str) -> usize {
    let grid = parse(input);

    let start = grid
        .coords_filter(|t| *t == Tile::Start)
        .next()
        .expect("missing start");

    count_timelines(&grid, start, &mut HashMap::new())
}

fn count_timelines(grid: &Matrix<Tile>, p: Point, memo: &mut HashMap<Point, usize>) -> usize {
    if let Some(&cached) = memo.get(&p) {
        return cached;
    }

    match grid.down(p) {
        None => {
            memo.insert(p, 1);
            1
        }
        Some(Tile::Empty) => {
            let count = count_timelines(grid, (p.0 + 1, p.1), memo);
            memo.insert(p, count);
            count
        }
        Some(Tile::Splitter) => {
            let left = count_timelines(grid, (p.0, p.1 - 1), memo);
            let right = count_timelines(grid, (p.0, p.1 + 1), memo);
            let total = left + right;
            memo.insert(p, total);
            total
        }
        Some(Tile::Start) => unreachable!("hit other start"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

        let result = laboratories(input);
        assert_eq!(40, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = laboratories(input);
        assert_eq!(32982105837605, result);
    }
}
