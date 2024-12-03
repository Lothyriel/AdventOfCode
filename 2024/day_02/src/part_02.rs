use crate::{is_safe_report, parse};

pub fn red_nosed_report(input: &str) -> usize {
    let report = parse(input);

    report.iter().filter(|l| is_safe_report_dampened(l)).count()
}

fn is_safe_report_dampened(report: &[u8]) -> bool {
    if is_safe_report(report) {
        return true;
    };

    for i in 0..report.len() {
        let mut t = report.to_vec();

        t.remove(i);

        if is_safe_report(&t) {
            return true;
        }
    }

    false
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

        assert_eq!(4, red_nosed_report(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(682, red_nosed_report(input));
    }
}
