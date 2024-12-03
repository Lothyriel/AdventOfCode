use std::{iter::Peekable, str::Bytes};

pub mod part_1;

type Queue<'a> = Peekable<Bytes<'a>>;

fn parse(input: &str) -> Vec<Multiplication> {
    let mut multiplications = Vec::new();

    let mut tokens = input.bytes().peekable();

    while let Some(t) = tokens.peek() {
        let o = match t {
            b'm' => try_parse_multiplication(&mut tokens),
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
        false => {
            tokens.next();
            None
        }
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

struct Multiplication(u32, u32);
