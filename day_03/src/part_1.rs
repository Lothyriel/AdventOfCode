use std::collections::VecDeque;

pub fn get_schematic_value(input: &str) -> usize {
    let tokens: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| parse_line(i, l))
        .collect();

    let symbol_positions: Vec<_> = tokens
        .iter()
        .filter_map(|t| match t.1 {
            Token::Symbol(_) => Some(t.0),
            Token::Number(_) => None,
        })
        .collect();

    let numbers = tokens.into_iter().filter_map(|t| match t.1 {
        Token::Number(n) => Some((t.0, n)),
        Token::Symbol(_) => None,
    });

    numbers
        .filter(|n| valid_parts(&symbol_positions, n))
        .map(|p| p.1.parse::<usize>().expect("Should be a number"))
        .sum()
}

fn valid_parts(symbols: &[Position], number: &(Position, String)) -> bool {
    symbols
        .iter()
        .flat_map(|s| (0..number.1.len()).map(|n| valid(number.0, n, s)))
        .any(|x| x)
}

const BORDER_OFFSETS: [(i32, i32); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn valid(position: Position, n: usize, symbol: &(usize, usize)) -> bool {
    let (line, col) = position;

    BORDER_OFFSETS
        .iter()
        .map(|r| (line as i32 + r.0, r.1 + col as i32 + n as i32))
        .any(|b| (b.0, b.1) == (symbol.0 as i32, symbol.1 as i32))
}

fn parse_line(line: usize, input: &str) -> Vec<(Position, Token)> {
    let mut input: VecDeque<_> = input.bytes().enumerate().collect();
    let mut tokens = Vec::new();

    while !input.is_empty() {
        if let Some(t) = parse(line, &mut input) {
            tokens.push(t);
        }
    }

    tokens
}

fn parse(line: usize, input: &mut VecDeque<(usize, u8)>) -> Option<(Position, Token)> {
    let (_, b) = input.front()?;

    match *b {
        b'0'..=b'9' => {
            let (c, n) = parse_number(input);
            Some(((line, c), n))
        }
        b'.' => {
            input.pop_front()?;
            None
        }
        _ => {
            let (i, b) = input.pop_front()?;
            let position = (line, i);
            Some((position, Token::Symbol(b)))
        }
    }
}

fn parse_number(input: &mut VecDeque<(usize, u8)>) -> (usize, Token) {
    let mut digits = Vec::new();

    while let Some(t) = input.front() {
        if t.1.is_ascii_digit() {
            let d = input.pop_front().expect("Should have more digits");
            digits.push(d);
        } else {
            break;
        }
    }

    let n = String::from_utf8(digits.iter().map(|t| t.1).collect()).expect("Should be valid UTF-8");

    let column = digits.first().expect("Should have a first digit").0;

    (column, Token::Number(n))
}

type Position = (usize, usize);

#[derive(PartialEq, Debug)]
enum Token {
    Number(String),
    Symbol(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_first_line() {
        let result = parse_line(0, "467..114..");

        let expected = vec![
            ((0, 0), Token::Number("467".to_string())),
            ((0, 5), Token::Number("114".to_string())),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn parsing_second_line() {
        let result = parse_line(1, "...*......");

        let expected = vec![((1, 3), Token::Symbol(b'*'))];

        assert_eq!(result, expected);
    }

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
