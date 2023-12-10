pub fn get_historical_value(input: &str) -> i64 {
    input.lines().map(get_value).sum()
}

fn get_value(input: &str) -> i64 {
    let history = input
        .split_whitespace()
        .flat_map(|n| n.parse())
        .rev()
        .collect();

    crate::predict_value(0, history)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_historical_value(include_str!("example"));
        assert_eq!(result, 2);
    }

    #[test]
    fn puzzle() {
        let result = get_historical_value(include_str!("input"));
        assert_eq!(result, 1053);
    }
}
