use crate::{Range, parse};

pub fn gift_shop(input: &str) -> usize {
    parse(input).map(|r| sum_invalid(&r)).sum()
}

fn sum_invalid(range: &Range) -> usize {
    (range.first_id..=range.last_id)
        .filter(|id| is_invalid(*id))
        .sum()
}

fn is_invalid(id: usize) -> bool {
    let digits = id.to_string();
    let bytes = digits.as_bytes();

    for d in 1..=digits.len() / 2 {
        if digits.len() % d != 0 {
            continue;
        }

        let sequence = &bytes[..d];

        let invalid = bytes[d..].chunks(d).all(|chunk| chunk == sequence);

        if invalid {
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
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        let result = gift_shop(input);
        assert_eq!(4174379265, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = gift_shop(input);
        assert_eq!(36862281418, result);
    }
}
