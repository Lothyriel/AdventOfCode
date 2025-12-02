use crate::{Range, parse};

pub fn gift_shop(input: &str) -> usize {
    parse(input).map(|r| sum_invalid(&r)).sum()
}

fn sum_invalid(range: &Range) -> usize {
    let mut count = 0;

    for id in range.first_id..=range.last_id {
        let digits = id.to_string();

        if digits.len() % 2 != 0 {
            continue;
        }

        let (start, end) = digits.split_at(digits.len() / 2);

        if start == end {
            count += id;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        let result = gift_shop(input);
        assert_eq!(1227775554, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = gift_shop(input);
        assert_eq!(19605500130, result);
    }
}
