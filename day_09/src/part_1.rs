pub fn get_historical_value(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_historical_value(include_str!("example"));
        assert_eq!(result, 114);
    }

    #[test]
    fn puzzle() {
        let result = get_historical_value(include_str!("input"));
        assert_eq!(result, 0);
    }
}
