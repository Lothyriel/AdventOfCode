use crate::{get_codes_complexity, parse};

pub fn keypad_conundrum(input: &str) -> usize {
    let codes = parse(input);

    get_codes_complexity(&codes, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let input = include_str!("input");

        let result = keypad_conundrum(input);
        assert_eq!(245881705840972, result);
    }
}
