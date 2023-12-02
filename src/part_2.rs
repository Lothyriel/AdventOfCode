pub fn get_calibration_value(input: &str) -> usize {
    input.lines().map(get_calibration_line).sum()
}

fn get_calibration_line(input: &str) -> usize {
    let first = {
        let mut idx = 0;

        loop {
            if let Ok(v) = parse(&mut input.bytes().skip(idx)) {
                break v;
            }

            idx += 1;
        }
    };

    let last = {
        let mut idx = 0;
        let mut last = first;

        loop {
            let a = parse(&mut input.bytes().skip(idx));
            idx += 1;
            match a {
                Ok(v) => last = v,
                Err(e) => match e {
                    State::NotNumber(_) => continue,
                    State::End => break last,
                },
            }
        }
    };

    first * 10 + last
}

fn parse(bytes: &mut impl Iterator<Item = u8>) -> Result<usize, State> {
    let mut c = || bytes.next().ok_or(State::End);

    let first = c()?;

    match first {
        b'0'..=b'9' => Ok(first as usize - 48),
        b'o' => match c()? {
            b'n' => match c()? {
                b'e' => Ok(1),
                n => Err(State::NotNumber(n as char)),
            },
            n => Err(State::NotNumber(n as char)),
        },
        b't' => match c()? {
            b'w' => match c()? {
                b'o' => Ok(2),
                n => Err(State::NotNumber(n as char)),
            },
            b'h' => match c()? {
                b'r' => match c()? {
                    b'e' => match c()? {
                        b'e' => Ok(3),
                        n => Err(State::NotNumber(n as char)),
                    },
                    n => Err(State::NotNumber(n as char)),
                },
                n => Err(State::NotNumber(n as char)),
            },
            n => Err(State::NotNumber(n as char)),
        },
        b'f' => match c()? {
            b'o' => match c()? {
                b'u' => match c()? {
                    b'r' => Ok(4),
                    n => Err(State::NotNumber(n as char)),
                },
                n => Err(State::NotNumber(n as char)),
            },
            b'i' => match c()? {
                b'v' => match c()? {
                    b'e' => Ok(5),
                    n => Err(State::NotNumber(n as char)),
                },
                n => Err(State::NotNumber(n as char)),
            },
            n => Err(State::NotNumber(n as char)),
        },
        b's' => match c()? {
            b'i' => match c()? {
                b'x' => Ok(6),
                n => Err(State::NotNumber(n as char)),
            },
            b'e' => match c()? {
                b'v' => match c()? {
                    b'e' => match c()? {
                        b'n' => Ok(7),
                        n => Err(State::NotNumber(n as char)),
                    },
                    n => Err(State::NotNumber(n as char)),
                },
                n => Err(State::NotNumber(n as char)),
            },
            n => Err(State::NotNumber(n as char)),
        },
        b'e' => match c()? {
            b'i' => match c()? {
                b'g' => match c()? {
                    b'h' => match c()? {
                        b't' => Ok(8),
                        n => Err(State::NotNumber(n as char)),
                    },
                    n => Err(State::NotNumber(n as char)),
                },
                n => Err(State::NotNumber(n as char)),
            },
            n => Err(State::NotNumber(n as char)),
        },
        b'n' => match c()? {
            b'i' => match c()? {
                b'n' => match c()? {
                    b'e' => Ok(9),
                    n => Err(State::NotNumber(n as char)),
                },
                n => Err(State::NotNumber(n as char)),
            },
            n => Err(State::NotNumber(n as char)),
        },
        n => Err(State::NotNumber(n as char)),
    }
}

#[derive(Debug)]
pub enum State {
    NotNumber(char),
    End,
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
    fn puzzle() {
        let input = include_str!("input");

        let value = get_calibration_value(input);

        assert_eq!(value, 0);
    }
}
