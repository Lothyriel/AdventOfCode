pub fn get_lowest_location(input: &str) -> usize {
    let almanac = std::sync::Arc::new(crate::FarmerAlmanac::parse(input));
    almanac.get_lowest_location_range()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("example");

        let result = get_lowest_location(input);

        assert_eq!(result, 46);
    }
}
