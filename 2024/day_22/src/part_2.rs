use std::collections::HashMap;

use crate::{generate_next, parse};

pub fn monkey_market(input: &str) -> usize {
    let mut indicators = HashMap::new();

    let numbers = parse(input).map(|n| generate_nth_numbers(n, 2000));

    for diffs in numbers {
        for (key, value) in get_indicators(&diffs) {
            indicators
                .entry(key)
                .and_modify(|v| *v += value as usize)
                .or_insert(value as usize);
        }
    }

    indicators.into_values().max_by_key(|&v| v).expect("Max")
}

fn get_indicators(d: &[(u8, i8)]) -> HashMap<(i8, i8, i8, i8), u8> {
    let mut map = HashMap::new();

    for (i, _) in d.iter().enumerate().skip(3) {
        let key = (d[i - 3].1, d[i - 2].1, d[i - 1].1, d[i].1);

        map.entry(key).or_insert(d[i].0);
    }

    map
}

fn generate_nth_numbers(n: usize, nth: usize) -> Vec<(u8, i8)> {
    let mut output = vec![];
    let mut current = n;

    for _ in 0..nth {
        let previous = current;
        current = generate_next(current);

        let diff = last_digit(current) - last_digit(previous);

        output.push((last_digit(current) as u8, diff));
    }

    output
}

fn last_digit(n: usize) -> i8 {
    (n % 10) as i8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_buyer_example() {
        let prices = generate_nth_numbers(123, 10);

        let indicators = get_indicators(&prices);

        let biggest_profit = indicators.into_values().max_by_key(|&v| v);

        assert_eq!(Some(6), biggest_profit);
    }

    #[test]
    fn example() {
        let input = r#"1
2
3
2024"#;

        assert_eq!(23, monkey_market(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(1690, monkey_market(input));
    }
}
