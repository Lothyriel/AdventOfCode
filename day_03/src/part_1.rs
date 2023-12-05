use crate::{adjacent, get_tokens, Token};

pub fn get_schematic_value(input: &str) -> usize {
    let tokens = get_tokens(input);

    let symbol_positions: Vec<_> = tokens
        .iter()
        .filter_map(|t| match t.1 {
            Token::Symbol(_) => Some(t.0),
            _ => None,
        })
        .collect();

    let numbers = tokens.into_iter().filter_map(|t| match t.1 {
        Token::Number(n) => Some((t.0, n)),
        _ => None,
    });

    numbers
        .filter(|n| adjacent(&symbol_positions, n).any(|_| true))
        .map(|p| p.1.parse::<usize>().expect("Should be a number"))
        .sum()
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

        let result = get_schematic_value(input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let result = get_schematic_value(input);

        assert_eq!(result, 540212);
    }
}
