use std::collections::HashMap;

use crate::{adjacent, get_tokens, Token};

pub fn get_gears_value(input: &str) -> usize {
    let tokens = get_tokens(input);

    let symbol_positions: Vec<_> = tokens
        .iter()
        .filter_map(|t| match t.1 {
            Token::Symbol(s) if s == b'*' => Some(t.0),
            _ => None,
        })
        .collect();

    let numbers = tokens.into_iter().filter_map(|t| match t.1 {
        Token::Number(n) => Some((t.0, n)),
        _ => None,
    });

    let adjacencies = numbers
        .filter_map(|n| adjacent(&symbol_positions, &n).map(|a| (n, a)))
        .fold(HashMap::new(), |mut acc, ((_, n), s)| {
            let entry = acc.entry(s).or_insert(Vec::new());

            entry.push(n.parse().expect("Should be a number"));

            acc
        });

    let gears = adjacencies.iter().filter(|a| a.1.len() > 1);

    gears.map(|g| g.1.iter().product::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let result = get_gears_value(input);

        assert_eq!(result, 467835);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let result = get_gears_value(input);

        assert_eq!(result, 87605697);
    }
}
