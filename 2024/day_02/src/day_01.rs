use crate::{is_safe_report, parse};

pub fn red_nosed_report(input: &str) -> usize {
    let report = parse(input);

    report.iter().filter(|l| is_safe_report(l)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(2, red_nosed_report(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(680, red_nosed_report(input));
    }
}
