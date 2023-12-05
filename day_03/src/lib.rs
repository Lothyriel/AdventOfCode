use std::collections::VecDeque;

pub mod part_1;
pub mod part_2;

fn get_tokens(input: &str) -> Vec<((usize, usize), Token)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| parse_line(i, l))
        .collect()
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
}
