use crate::Token;

pub fn mull_it_over(input: &str) -> u32 {
    let tokens = crate::parse(input);

    tokens
        .iter()
        .filter_map(|t| match t {
            Token::Mul(m) => Some(m),
            _ => None,
        })
        .map(|m| m.0 * m.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(161, mull_it_over(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(173419328, mull_it_over(input));
    }
}
