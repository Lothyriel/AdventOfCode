pub fn get_lowest_location(input: &str) -> usize {
    crate::FarmerAlmanac::parse(input).get_lowest_location()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("example");

        let result = get_lowest_location(input);

        assert_eq!(result, 35);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let result = get_lowest_location(input);

        assert_eq!(result, 88151870);
    }
}
