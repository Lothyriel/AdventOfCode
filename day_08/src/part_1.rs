pub fn get_steps_count(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_steps_count(include_str!("example"));
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = get_steps_count(include_str!("example_2"));
        assert_eq!(result, 6);
    }

    #[test]
    fn puzzle() {
        let result = get_steps_count(include_str!("example"));
        assert_eq!(result, 0);
    }
}
