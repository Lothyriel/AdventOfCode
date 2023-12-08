use crate::{FarmerAlmanac, Mapper};

pub fn get_lowest_location(input: &str) -> usize {
    FarmerAlmanac::parse(input).get_lowest_location()
}

impl FarmerAlmanac {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split(':');

        let seeds = parts
            .nth(1)
            .expect("Should contain seeds part")
            .split_whitespace()
            .flat_map(|n| n.parse())
            .collect();

        let mappers = parts.map(Mapper::parse).collect();

        Self { seeds, mappers }
    }
}

#[cfg(test)]
mod tests {
    use crate::Range;

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

    #[test]
    fn parse_example_almanac() {
        let expected = FarmerAlmanac {
            mappers: vec![
                Mapper {
                    ranges: vec![
                        Range {
                            dest: 50,
                            source: 98,
                            length: 2,
                        },
                        Range {
                            dest: 52,
                            source: 50,
                            length: 48,
                        },
                    ],
                },
                Mapper {
                    ranges: vec![
                        Range {
                            dest: 0,
                            source: 15,
                            length: 37,
                        },
                        Range {
                            dest: 37,
                            source: 52,
                            length: 2,
                        },
                        Range {
                            dest: 39,
                            source: 0,
                            length: 15,
                        },
                    ],
                },
                Mapper {
                    ranges: vec![
                        Range {
                            dest: 49,
                            source: 53,
                            length: 8,
                        },
                        Range {
                            dest: 0,
                            source: 11,
                            length: 42,
                        },
                        Range {
                            dest: 42,
                            source: 0,
                            length: 7,
                        },
                        Range {
                            dest: 57,
                            source: 7,
                            length: 4,
                        },
                    ],
                },
                Mapper {
                    ranges: vec![
                        Range {
                            dest: 88,
                            source: 18,
                            length: 7,
                        },
                        Range {
                            dest: 18,
                            source: 25,
                            length: 70,
                        },
                    ],
                },
                Mapper {
                    ranges: vec![
                        Range {
                            dest: 45,
                            source: 77,
                            length: 23,
                        },
                        Range {
                            dest: 81,
                            source: 45,
                            length: 19,
                        },
                        Range {
                            dest: 68,
                            source: 64,
                            length: 13,
                        },
                    ],
                },
                Mapper {
                    ranges: vec![
                        Range {
                            dest: 0,
                            source: 69,
                            length: 1,
                        },
                        Range {
                            dest: 1,
                            source: 0,
                            length: 69,
                        },
                    ],
                },
                Mapper {
                    ranges: vec![
                        Range {
                            dest: 60,
                            source: 56,
                            length: 37,
                        },
                        Range {
                            dest: 56,
                            source: 93,
                            length: 4,
                        },
                    ],
                },
            ],
            seeds: vec![79, 14, 55, 13],
        };

        assert_eq!(FarmerAlmanac::parse(include_str!("example")), expected);
    }
}
