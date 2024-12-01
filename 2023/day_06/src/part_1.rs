use crate::{get_winning_strategies, Record};

pub fn get_record_value(input: &str) -> usize {
    parse_records(input)
        .map(|r| get_winning_strategies(r).count())
        .product()
}

fn parse_records(input: &str) -> impl Iterator<Item = Record> + '_ {
    let mut lines = input.lines();

    let timmings = lines.next().expect("Expected timmings line");

    let distances = lines.next().expect("Expected distances lines");

    to_numbers(timmings)
        .zip(to_numbers(distances))
        .map(|(time, distance)| Record { time, distance })
}

fn to_numbers(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split_whitespace().flat_map(|n| n.parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(get_record_value(include_str!("example")), 288);
    }

    #[test]
    fn puzzle() {
        assert_eq!(get_record_value(include_str!("input")), 138915);
    }
}
