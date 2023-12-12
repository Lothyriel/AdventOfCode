#[cfg(test)]
mod tests {
    #[test]
    fn example_10() {
        let result = crate::solve(include_str!("example"), 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn example_100() {
        let result = crate::solve(include_str!("example"), 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn input() {
        let result = crate::solve(include_str!("input"), 1_000_000);
        assert_eq!(result, 827009909817);
    }
}
