use crate::{get_tokens, Position, Token, BORDER_OFFSETS};

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
        .filter(|n| adjacent(&symbol_positions, n))
        .map(|p| p.1.parse::<usize>().expect("Should be a number"))
        .sum()
}

fn adjacent(symbols: &[Position], number: &(Position, String)) -> bool {
    symbols
        .iter()
        .flat_map(|s| (0..number.1.len()).map(|n| adjacent_offset(number.0, n, s)))
        .any(|x| x)
}

fn adjacent_offset(position: Position, n: usize, symbol: &(usize, usize)) -> bool {
    let (line, col) = position;

    BORDER_OFFSETS
        .iter()
        .map(|r| (line as i32 + r.0, r.1 + col as i32 + n as i32))
        .any(|b| (b.0, b.1) == (symbol.0 as i32, symbol.1 as i32))
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
