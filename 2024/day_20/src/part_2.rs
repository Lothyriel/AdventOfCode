use crate::cheating_shortcuts;

pub fn race_condition(input: &str) -> usize {
    cheating_shortcuts(input, 20)
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

        let result = cheating_shortcuts(input, 20);

        assert_eq!(32, result[&50]);
        assert_eq!(31, result[&52]);
        assert_eq!(29, result[&54]);
        assert_eq!(39, result[&56]);
        assert_eq!(25, result[&58]);
        assert_eq!(23, result[&60]);
        assert_eq!(20, result[&62]);
        assert_eq!(19, result[&64]);
        assert_eq!(12, result[&66]);
        assert_eq!(14, result[&68]);
        assert_eq!(12, result[&70]);
        assert_eq!(22, result[&72]);
        assert_eq!(4, result[&74]);
        assert_eq!(3, result[&76]);
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(999556, race_condition(input));
    }
}
