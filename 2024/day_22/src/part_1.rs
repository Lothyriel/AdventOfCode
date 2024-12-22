use crate::{generate_next, parse};

pub fn monkey_market(input: &str) -> usize {
    parse(input).map(|n| generate_nth(n, 2000)).sum()
}

fn generate_nth(mut n: usize, nth: usize) -> usize {
    for _ in 0..nth {
        n = generate_next(n);
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"1
10
100
2024"#;

        assert_eq!(37327623, monkey_market(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(14180628689, monkey_market(input));
    }
}
