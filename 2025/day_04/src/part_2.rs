use crate::{Tile, parse};

pub fn printing_department(input: &str) -> usize {
    let mut grid = parse(input);

    let mut removed = 0;

    loop {
        let mut reachable = Vec::new();

        for (x, y) in grid.coords_filter(|t| *t == Tile::PaperRoll) {
            let neighbours = grid
                .adjacent((x, y))
                .filter(|n| **n == Tile::PaperRoll)
                .count();

            if neighbours < 4 {
                reachable.push((x, y));
            }
        }

        if reachable.is_empty() {
            break removed;
        }

        for point in reachable.drain(..) {
            grid.set(point, Tile::Empty);
            removed += 1;
        }
    }
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
        assert_eq!(43, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = printing_department(input);
        assert_eq!(8623, result);
    }
}
