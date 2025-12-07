use std::collections::HashSet;

use crate::{Tile, parse};

pub fn laboratories(input: &str) -> usize {
    let grid = parse(input);

    let mut beams: Vec<_> = grid.coords_filter(|t| *t == Tile::Start).collect();
    let mut unique = HashSet::new();
    let mut splits = 0;

    loop {
        let Some(b) = beams.pop() else {
            break;
        };

        let mut add = |point| {
            if unique.insert(point) {
                beams.push(point);
            };
        };

        let Some(down) = grid.down(b) else {
            continue;
        };

        match down {
            Tile::Splitter => {
                splits += 1;
                add((b.0 + 1, b.1 - 1));
                add((b.0 + 1, b.1 + 1));
            }
            Tile::Empty => add((b.0 + 1, b.1)),
            Tile::Start => unreachable!("hit other start"),
        }
    }

    splits
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
        assert_eq!(21, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = laboratories(input);
        assert_eq!(1524, result);
    }
}
