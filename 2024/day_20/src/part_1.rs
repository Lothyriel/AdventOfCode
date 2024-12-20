use crate::cheating_shortcuts;

pub fn race_condition(input: &str) -> usize {
    cheating_shortcuts(input, 2)
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| v)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

        let result = cheating_shortcuts(input, 2);

        assert_eq!(14, result[&2]);
        assert_eq!(14, result[&4]);
        assert_eq!(2, result[&6]);
        assert_eq!(4, result[&8]);
        assert_eq!(2, result[&10]);
        assert_eq!(3, result[&12]);
        assert_eq!(1, result[&20]);
        assert_eq!(1, result[&36]);
        assert_eq!(1, result[&36]);
        assert_eq!(1, result[&40]);
        assert_eq!(1, result[&64]);
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(1452, race_condition(input));
    }
}
