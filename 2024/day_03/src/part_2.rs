use crate::Token;

pub fn mull_it_over(input: &str) -> u32 {
    let tokens = crate::parse(input);

    let (_, sum) = tokens.iter().fold((true, 0), |(mut enabled, mut sum), t| {
        match t {
            Token::Mul(m) => {
                if enabled {
                    sum += m.0 * m.1
                }
            }
            Token::Dont => enabled = false,
            Token::Do => enabled = true,
        };

        (enabled, sum)
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(48, mull_it_over(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(90669332, mull_it_over(input));
    }
}
