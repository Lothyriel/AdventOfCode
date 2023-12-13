use std::collections::VecDeque;

pub fn get_total_arrangements(input: &str) -> usize {
    input.lines().map(parse).map(arrangements).sum()
}

fn arrangements(record: ConditionRecord) -> usize {
    let unknowns = record
        .springs
        .iter()
        .filter(|s| matches!(s, Spring::Unknown))
        .count();

    get_permutations(unknowns)
        .iter()
        .filter(|p| valid_perm(&record, p))
        .count()
}

fn valid_perm(record: &ConditionRecord, perm: &[Spring]) -> bool {
    let mut perm = perm.iter();

    let mut new: VecDeque<_> = record
        .springs
        .iter()
        .map(|s| match s {
            Spring::Unknown => perm.next().expect("Should contain next"),
            _ => s,
        })
        .collect();

    let mut config = Vec::new();

    while let Some(s) = new.front() {
        match s {
            Spring::Damaged => config.push(parse_damaged(&mut new)),
            Spring::Operational => {
                new.pop_front();
            }
            Spring::Unknown => panic!("Should not have unknowns by now"),
        }
    }

    config == record.config
}

fn parse_damaged(new: &mut VecDeque<&Spring>) -> usize {
    let mut count = 0;

    while let Some(o) = new.pop_front() {
        if matches!(o, Spring::Damaged) {
            count += 1;
        } else {
            break;
        }
    }

    count
}

fn get_permutations(unknowns: usize) -> Vec<Vec<Spring>> {
    (0..1 << unknowns)
        .map(|b| {
            (0..unknowns)
                .map(|i| (b & 1 << i) >> i)
                .map(|b| match b {
                    0 => Spring::Damaged,
                    1 => Spring::Operational,
                    _ => panic!("Invalid bit"),
                })
                .collect()
        })
        .collect()
}

fn parse(input: &str) -> ConditionRecord {
    let mut parts = input.split_whitespace();

    let springs = parts
        .next()
        .expect("Expected springs part")
        .bytes()
        .map(|b| match b {
            b'.' => Spring::Operational,
            b'?' => Spring::Unknown,
            b'#' => Spring::Damaged,
            c => panic!("Invalid character {}", c as char),
        })
        .collect();

    let config = parts
        .next()
        .expect("Expected config part")
        .split(',')
        .flat_map(|n| n.parse())
        .collect();

    ConditionRecord { springs, config }
}

struct ConditionRecord {
    springs: Vec<Spring>,
    config: Vec<usize>,
}

#[derive(Debug)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perm_validation() {
        let record = parse("???.### 1,1,3");

        let arrangements = arrangements(record);

        assert_eq!(arrangements, 1);
    }

    #[test]
    fn example() {
        let result = get_total_arrangements(include_str!("example"));
        assert_eq!(result, 21);
    }

    #[test]
    fn input() {
        let result = get_total_arrangements(include_str!("input"));
        assert_eq!(result, 7173);
    }
}
