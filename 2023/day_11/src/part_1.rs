pub fn get_distances_sum(input: &str) -> isize {
    crate::solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_distances_sum(include_str!("example"));
        assert_eq!(result, 374);
    }

    #[test]
    fn input() {
        let result = get_distances_sum(include_str!("input"));
        assert_eq!(result, 9563821);
    }
}
