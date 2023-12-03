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

fn parse(input: &mut VecDeque<u8>) -> Option<Token> {
    let consume = |i: &mut VecDeque<u8>| {
        i.pop_front()?;
        None
    };

    let peek = |i: &mut VecDeque<u8>| match i.get(1) {
        Some(t) => Some(*t),
        None => {
            consume(i);
            None
        }
    };
    match input.front()? {
        b'0'..=b'9' => Some(Token::Number(input.pop_front()? as usize - 48)),
        b'o' => try_parse("one", input),
        b't' => match peek(input)? {
            b'w' => try_parse("two", input),
            b'h' => try_parse("three", input),
            _ => consume(input),
        },
        b'f' => match peek(input)? {
            b'o' => try_parse("four", input),
            b'i' => try_parse("five", input),
            _ => consume(input),
        },
        b's' => match peek(input)? {
            b'i' => try_parse("six", input),
            b'e' => try_parse("seven", input),
            _ => consume(input),
        },
        b'e' => try_parse("eight", input),
        b'n' => try_parse("nine", input),
        _ => consume(input),
    }
}

fn try_parse(expected: &str, input: &mut VecDeque<u8>) -> Option<Token> {
    let mut head = u8::MAX;

    for b in expected.bytes() {
        head = input.pop_front()?;
        if b != head {
            input.push_front(head);
            return None;
        }
    }

    input.push_front(head);

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
