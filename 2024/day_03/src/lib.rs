use std::{iter::Peekable, str::Bytes};

pub mod part_1;
pub mod part_2;

type Queue<'a> = Peekable<Bytes<'a>>;

fn parse(input: &str) -> Vec<Token> {
    let mut multiplications = Vec::new();

    let mut tokens = input.bytes().peekable();

    while let Some(t) = tokens.peek() {
        let o = match t {
            b'm' => try_parse_multiplication(&mut tokens).map(Token::Mul),
            b'd' => try_parse_modifier(&mut tokens),
            _ => {
                tokens.next();
                None
            }
        };

        if let Some(o) = o {
            multiplications.push(o);
        }
    }

    multiplications
}

fn try_parse_modifier(tokens: &mut Queue) -> Option<Token> {
    consume(tokens, |t| t == b'd')?;
    consume(tokens, |t| t == b'o')?;

    let next = tokens.peek()?;

    match next {
        b'(' => try_parse_do(tokens),
        b'n' => try_parse_dont(tokens),
        _ => None,
    }
}

fn try_parse_dont(tokens: &mut Queue) -> Option<Token> {
    consume(tokens, |t| t == b'n')?;
    consume(tokens, |t| t == b'\'')?;
    consume(tokens, |t| t == b't')?;

    Some(Token::Dont)
}

fn try_parse_do(tokens: &mut Queue) -> Option<Token> {
    consume(tokens, |t| t == b'(')?;
    consume(tokens, |t| t == b')')?;

    Some(Token::Do)
}

fn try_parse_multiplication(tokens: &mut Queue) -> Option<Multiplication> {
    consume(tokens, |t| t == b'm')?;
    consume(tokens, |t| t == b'u')?;
    consume(tokens, |t| t == b'l')?;

    consume(tokens, |t| t == b'(')?;

    let first = consume(tokens, |t| t.is_ascii_digit())?;
    consume(tokens, |t| t == b',')?;
    let second = consume(tokens, |t| t.is_ascii_digit())?;

    consume(tokens, |t| t == b')')?;

    Some(Multiplication(first, second))
}

fn consume(tokens: &mut Queue, predicate: impl Fn(u8) -> bool) -> Option<u32> {
    let token = tokens.peek()?;

    match predicate(*token) {
        true => {
            let number = if token.is_ascii_digit() {
                parse_number(tokens)
            } else {
                tokens.next();
                0
            };

            Some(number)
        }
        false => None,
    }
}

fn parse_number(tokens: &mut Queue) -> u32 {
    let mut number = String::new();

    while let Some(t) = tokens.peek() {
        if t.is_ascii_digit() {
            number.push(*t as char);
            tokens.next();
        } else {
            break;
        }
    }

    number.parse().expect("Number")
}

enum Token {
    Do,
    Dont,
    Mul(Multiplication),
}

struct Multiplication(u32, u32);
