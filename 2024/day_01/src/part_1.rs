pub fn historian_hysteria(input: &str) -> u64 {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    left.iter().zip(right).map(|x| x.0.abs_diff(x.1)).sum()
}

pub fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut s| (s.next().expect("First"), s.next().expect("Second")))
        .map(|(first, second)| {
            (
                first.parse().expect("Number"),
                second.parse().expect("Number"),
            )
        })
        .fold((Vec::new(), Vec::new()), |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);

            acc
        })
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

        assert_eq!(11, historian_hysteria(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(2815556, historian_hysteria(input));
    }
}
