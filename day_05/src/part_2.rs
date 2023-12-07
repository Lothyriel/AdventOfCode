use crate::{FarmerAlmanac, Mapper};

pub fn get_lowest_location(input: &str) -> usize {
    FarmerAlmanac::parse_2(input).get_lowest_location()
}

impl FarmerAlmanac {
    pub fn parse_2(input: &str) -> Self {
        let mut parts = input.split(':');

        let seeds: Vec<_> = parts
            .nth(1)
            .expect("Should contain seeds part")
            .split_whitespace()
            .collect();

        let seeds = seeds
            .chunks_exact(2)
            .flat_map(|n| {
                let mut parts = n.iter().flat_map(|n| n.parse());

                let start = parts.next()?;

                let range = parts.next()?;

                Some(start..range + start)
            })
            .flatten()
            .collect();

        let mappers = parts.map(Mapper::parse).collect();

        Self { seeds, mappers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_seed() {
        let expected: Vec<_> = (79..=92).chain(55..=67).collect();

        let seeds = FarmerAlmanac::parse_2(include_str!("example")).seeds;

        assert_eq!(seeds, expected);
        assert_eq!(seeds.len(), 27);
    }

    #[test]
    fn example() {
        let input = include_str!("example");

        let result = get_lowest_location(input);

        assert_eq!(result, 46);
    }
}
