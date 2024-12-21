use crate::{get_codes_complexity, parse};

pub fn keypad_conundrum(input: &str) -> usize {
    let codes = parse(input);

    get_codes_complexity(&codes, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"029A
980A
179A
456A
379A"#;

        let result = keypad_conundrum(input);
        assert_eq!(126384, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        let result = keypad_conundrum(input);
        assert_eq!(202274, result);
    }
}
