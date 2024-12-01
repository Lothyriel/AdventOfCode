pub fn get_calibration_value(input: &str) -> usize {
    input.lines().map(get_calibration_line).sum()
}

fn get_calibration_line(input: &str) -> usize {
    let mut numbers = input.bytes().filter_map(|b| match b {
        b'0'..=b'9' => Some(b as usize - 48),
        _ => None,
    });

    let first = numbers.next().expect("Expected at least a number");

    let last = numbers.next_back().unwrap_or(first);

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_lines() {
        assert_eq!(get_calibration_line("1abc2"), 12);
        assert_eq!(get_calibration_line("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_line("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_line("treb7uchet"), 77);
        assert_eq!(get_calibration_line("9s"), 99);
    }

    #[test]
    fn example() {
        let input = r#"1abc2
                       pqr3stu8vwx
                       a1b2c3d4e5f
                       treb7uchet"#;

        let value = get_calibration_value(input);

        assert_eq!(value, 142);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let value = get_calibration_value(input);

        assert_eq!(value, 54450);
    }
}
