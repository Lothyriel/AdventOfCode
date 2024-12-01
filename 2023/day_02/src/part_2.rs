use crate::{parse_game, Game, Set};

pub fn get_min_config_value(input: &str) -> usize {
    input
        .lines()
        .map(parse_game)
        .map(get_game_min_set)
        .map(|s| s.0 * s.1 * s.2)
        .sum()
}

fn get_game_min_set(game: Game) -> Set {
    game.sets.iter().fold((0, 0, 0), |acc, s| {
        (acc.0.max(s.0), acc.1.max(s.1), acc.2.max(s.2))
    })
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

        assert_eq!(get_min_config_value(input), 2286);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        assert_eq!(get_min_config_value(input), 63542);
    }
}
