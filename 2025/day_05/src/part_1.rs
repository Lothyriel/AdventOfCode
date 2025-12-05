use crate::parse;

pub fn cafeteria(input: &str) -> usize {
    let (ranges, ingrediensts) = parse(input);

    ingrediensts
        .iter()
        .filter(|&&i| ranges.iter().any(|r| i >= r.start && i <= r.end))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        let result = cafeteria(input);
        assert_eq!(3, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = cafeteria(input);
        assert_eq!(726, result);
    }
}
