use crate::Map;

pub fn guard_gallivant(input: &str) -> usize {
    Map::parse(input).get_path().len()
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
        assert_eq!(41, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = guard_gallivant(input);
        assert_eq!(5461, result);
    }
}
