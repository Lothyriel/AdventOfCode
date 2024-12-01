use std::collections::HashMap;

use crate::parse;

pub fn historian_hysteria(input: &str) -> u64 {
    let (left, right) = parse(input);

    let occurrences = right.iter().fold(HashMap::new(), |mut acc, x| {
        let occurrences = acc.entry(x).or_insert(0);

        *occurrences += 1;

        acc
    });

    left.iter()
        .map(|x| x * occurrences.get(x).copied().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(31, historian_hysteria(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(23927637, historian_hysteria(input));
    }
}
