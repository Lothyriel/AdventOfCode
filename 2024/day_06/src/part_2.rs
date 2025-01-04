use crate::{Direction, Map, Tile};

pub fn guard_gallivant(input: &str) -> usize {
    let mut map = Map::parse(input);

    let start = map.guard_pos;

    let mut patroll_path = map.get_path();
    patroll_path.remove(&start);

    patroll_path
        .iter()
        .filter(|&&pos| {
            map.inner.set(pos, Tile::Obstacle);
            map.guard_pos = start;
            map.guard_dir = Direction::Up;

            let loops = map.patroll_loops();
            map.inner.set(pos, Tile::Empty);

            loops
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        let result = guard_gallivant(input);
        assert_eq!(6, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = guard_gallivant(input);
        assert_eq!(1836, result);
    }
}
