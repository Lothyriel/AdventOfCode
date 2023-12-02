pub fn get_calibration_value(input: &str) -> Result<usize, Error> {
    input.lines().map(get_calibration_line).sum()
}

fn get_calibration_line(input: &str) -> Result<usize, Error> {
    let mut numbers = input.bytes().filter(|b| b.is_ascii_digit());

    let first = to_digit(numbers.next())?;

    let last = to_digit(numbers.next_back()).unwrap_or(first);

    Ok(first * 10 + last)
}

fn to_digit(byte: Option<u8>) -> Result<usize, Error> {
    let byte = byte.ok_or(Error::NumberNotFound)?;

    match byte {
        b'0'..=b'9' => Ok(byte as usize - 48),
        _ => Err(Error::Parsing),
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Couldn't find a number in the input line")]
    NumberNotFound,
    #[error("Couldn't convert to a digit representation")]
    Parsing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_lines() {
        let input = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
            ("9s", 99),
        ];

        for (i, r) in input {
            let value = get_calibration_line(i);
            assert_eq!(value, Ok(r));
        }
    }

    #[test]
    fn example() {
        let input = r#"1abc2
                       pqr3stu8vwx
                       a1b2c3d4e5f
                       treb7uchet"#;

        let value = get_calibration_value(input);

        assert_eq!(value, Ok(142));
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let value = get_calibration_value(input);

        assert_eq!(value, Ok(54450));
    }
}
