use std::cmp::Ordering;

pub fn red_nosed_report(input: &str) -> usize {
    let report = parse(input);

    report.iter().filter(|l| is_safe_report(l)).count()
}

enum ReportStatus {
    Increasing,
    Decreasing,
}

fn is_safe_report(report: &[u8]) -> bool {
    let mut status = None;

    for w in report.windows(2) {
        if w[0].abs_diff(w[1]) > 3 {
            return false;
        }

        if let Some(s) = &status {
            let is_safe = match s {
                ReportStatus::Decreasing => w[0] > w[1],
                ReportStatus::Increasing => w[0] < w[1],
            };

            if !is_safe {
                return false;
            }
        } else {
            status = match w[0].cmp(&w[1]) {
                Ordering::Less => Some(ReportStatus::Increasing),
                Ordering::Equal => return false,
                Ordering::Greater => Some(ReportStatus::Decreasing),
            };
        }
    }

    true
}

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|l| l.map(|n| n.parse().expect("Number")).collect())
        .collect()
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
