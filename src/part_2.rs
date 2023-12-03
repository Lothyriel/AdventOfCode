use std::collections::VecDeque;

pub fn get_calibration_value(input: &str) -> usize {
    input.lines().map(get_calibration_line).sum()
}

fn get_calibration_line(input: &str) -> usize {
    let tokens = get_tokens(input.bytes().collect());

    let mut numbers = tokens.iter().filter_map(|t| match t {
        Token::Number(n) => Some(n),
        Token::Character(_) => None,
    });

    let first = numbers.next().expect("Expected at least one number");

    let last = numbers.next_back().unwrap_or(first);

    first * 10 + last
}

fn get_tokens(mut input: VecDeque<u8>) -> Vec<Token> {
    let mut tokens = Vec::new();

    while !input.is_empty() {
        if let Some(t) = parse(&mut input) {
            tokens.push(t);
        }
    }

    tokens
}

fn parse(bytes: &mut Bytes<'_>) -> Option<Token> {
    let first = bytes.next()?;

    match first {
        b'0'..=b'9' => Some(Token::Number(first as usize - 48)),
        b'o' => try_parse("ne", bytes),
        b't' => match bytes.peek()? {
            b'w' => try_parse("wo", bytes),
            b'h' => try_parse("hree", bytes),
            _ => None,
        },
        b'f' => match bytes.peek()? {
            b'o' => try_parse("our", bytes),
            b'i' => try_parse("ive", bytes),
            _ => None,
        },
        b's' => match bytes.peek()? {
            b'i' => try_parse("ix", bytes),
            b'e' => try_parse("even", bytes),
            _ => None,
        },
        b'e' => try_parse("ight", bytes),
        b'n' => try_parse("ine", bytes),
        _ => None,
    }
}

fn try_parse(expected: &str, bytes: &mut Bytes<'_>) -> Option<Token> {
    for e in expected.bytes() {
        // println!(
        //     "expected: {}, got: {:?}",
        //     e as char,
        //     bytes.peek().map(|c| *c as char)
        // );
        if e == *bytes.peek()? {
            bytes.next()?;
        } else {
            return None;
        }
    }


    let number = match expected {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Should match an number"),
    };

    Some(Token::Number(number))
}

#[derive(Debug)]
pub enum Token {
    Number(usize),
    Character(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_lines() {
        assert_eq!(get_calibration_line("two1nine"), 29);
        assert_eq!(get_calibration_line("eightwothree"), 83);
        assert_eq!(get_calibration_line("abcone2threexyz"), 13);
        assert_eq!(get_calibration_line("xtwone3four"), 24);
        assert_eq!(get_calibration_line("4nineeightseven2"), 42);
        assert_eq!(get_calibration_line("zoneight234"), 14);
        assert_eq!(get_calibration_line("7pqrstsixteen"), 76);
    }

    #[test]
    fn example() {
        let input = r#"two1nine
                       eightwothree
                       abcone2threexyz
                       xtwone3four
                       4nineeightseven2
                       zoneight234
                       7pqrstsixteen"#;

        let value = get_calibration_value(input);

        assert_eq!(value, 281);
    }

    #[test]
    fn example_numbers_sharing_letters() {
        assert_eq!(get_calibration_line("28gtbkszmrtmnineoneightmx"), 28);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let value = get_calibration_value(input);

        assert_eq!(value, 54265);
    }
}
