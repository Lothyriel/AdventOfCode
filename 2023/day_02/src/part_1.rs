use crate::{parse_game, Set};

pub fn get_possible_games(input: &str, config: Set) -> usize {
    let valid = |s: &Set| s.0 <= config.0 && s.1 <= config.1 && s.2 <= config.2;

    input
        .lines()
        .map(parse_game)
        .filter(|g| g.sets.iter().all(valid))
        .map(|g| g.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let config = (12, 13, 14);

        assert_eq!(get_possible_games(input, config), 8);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let config = (12, 13, 14);

        assert_eq!(get_possible_games(input, config), 2268);
    }
}
