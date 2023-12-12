pub fn get_max_distance(input: &str) -> usize {
    crate::PipeMaze::parse(input).farthest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_max_distance(include_str!("example"));
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let result = get_max_distance(include_str!("example_2"));
        assert_eq!(result, 8);
    }

    #[test]
    fn puzzle() {
        let result = get_max_distance(include_str!("input"));
        assert_eq!(result, 6831);
    }
}
