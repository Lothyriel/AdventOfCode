use crate::{get_winning_strategies, Record};

pub fn get_record_value(input: &str) -> usize {
    let record = parse_record(input);

    get_winning_strategies(record).count()
}

fn parse_record(input: &str) -> Record {
    let mut lines = input.lines();

    let time = lines.next().expect("Expected time line");

    let distance = lines.next().expect("Expected distance lines");

    Record {
        time: to_number(time),
        distance: to_number(distance),
    }
}

fn to_number(i: &str) -> usize {
    i.bytes()
        .filter(u8::is_ascii_digit)
        .map(|d| d as usize - 48)
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10usize.pow(i as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(get_record_value(include_str!("example")), 71503);
    }

    #[test]
    fn puzzle() {
        assert_eq!(get_record_value(include_str!("input")), 27340847);
    }
}
