pub mod part_02;
pub mod part_1;

use std::cmp::Ordering;

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|l| l.map(|n| n.parse().expect("Number")).collect())
        .collect()
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
