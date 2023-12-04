pub fn get_possible_games(input: &str, config: Set) -> usize {
    let valid = |s: &Set| s.0 <= config.0 && s.1 <= config.1 && s.2 <= config.2;

    input
        .lines()
        .map(parse_game)
        .filter(|g| g.sets.iter().all(valid))
        .map(|g| g.id)
        .sum()
}

fn parse_game(input: &str) -> Game {
    let mut parts = input.split(':');

    let id = parse_id(parts.next().expect("Should contain game id part"));

    let sets = parse_sets(parts.next().expect("Should contain sets part"));

    Game { id, sets }
}

fn parse_id(id_part: &str) -> usize {
    let digits = id_part.bytes().filter(u8::is_ascii_digit);

    to_number(digits)
}

fn to_number(digits: impl DoubleEndedIterator<Item = u8>) -> usize {
    let c = |b| b as usize - 48;

    digits
        .rev()
        .enumerate()
        .fold(0, |acc, (i, d)| acc + c(d) * 10usize.pow(i as u32))
}

fn parse_sets(sets: &str) -> Vec<Set> {
    sets.split(';').map(parse_set).collect()
}

fn parse_set(set: &str) -> Set {
    let cubes: Vec<_> = set.split(',').map(parse_cube).collect();

    let f = |color: Color| {
        cubes
            .iter()
            .find(|c| c.1 == color)
            .map(|c| c.0)
            .unwrap_or(0)
    };

    (f(Color::Red), f(Color::Green), f(Color::Blue))
}

fn parse_cube(cube: &str) -> (usize, Color) {
    let mut set_parts = cube.split_whitespace();

    let number = set_parts.next().expect("Should contain a number");

    let number = to_number(number.bytes());

    let color = set_parts.next().expect("Should contain a color");

    let color = match color {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!("Should match a color"),
    };

    (number, color)
}

#[derive(PartialEq, Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

type Set = (usize, usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_id_test() {
        let input = "Game 123";

        assert_eq!(parse_id(input), 123);
    }

    #[test]
    fn parse_game_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let expected = Game {
            id: 1,
            sets: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)],
        };

        assert_eq!(parse_game(input), expected);
    }

    #[test]
    fn parse_game_2() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

        let expected = Game {
            id: 2,
            sets: vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)],
        };

        assert_eq!(parse_game(input), expected);
    }

    #[test]
    fn parse_game_3() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        let expected = Game {
            id: 3,
            sets: vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)],
        };

        assert_eq!(parse_game(input), expected);
    }

    #[test]
    fn parse_game_4() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";

        let expected = Game {
            id: 4,
            sets: vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)],
        };

        assert_eq!(parse_game(input), expected);
    }

    #[test]
    fn parse_game_5() {
        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let expected = Game {
            id: 5,
            sets: vec![(6, 3, 1), (1, 2, 2)],
        };

        assert_eq!(parse_game(input), expected);
    }

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
