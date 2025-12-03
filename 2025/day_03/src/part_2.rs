use crate::{get_largest_number, parse};

pub fn lobby(input: &str) -> usize {
    parse(input)
        .map(|b| get_largest_number(&b.batteries, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        let result = lobby(input);
        assert_eq!(3121910778619, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = lobby(input);
        assert_eq!(170997883706617, result);
    }
}
