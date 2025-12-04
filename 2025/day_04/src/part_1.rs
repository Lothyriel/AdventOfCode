use crate::{Tile, parse};

pub fn printing_department(input: &str) -> usize {
    let grid = parse(input);

    let mut accessible = 0;

    for (x, y) in grid.coords_filter(|t| *t == Tile::PaperRoll) {
        let neighbours = grid
            .adjacent((x, y))
            .filter(|n| **n == Tile::PaperRoll)
            .count();

        if neighbours < 4 {
            accessible += 1;
        }
    }

    accessible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        let result = printing_department(input);
        assert_eq!(13, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = printing_department(input);
        assert_eq!(1435, result);
    }
}
