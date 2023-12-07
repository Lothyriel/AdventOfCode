pub mod part_1;

#[derive(Debug, PartialEq)]
struct FarmerAlmanac {
    mappers: Vec<Mapper>,
    seeds: Vec<usize>,
}

impl FarmerAlmanac {
    fn parse(input: &str) -> Self {
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

    fn get_lowest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|s| self.map_seed(*s))
            .min()
            .expect("Should have a min value")
    }

    fn map_seed(&self, seed: usize) -> usize {
        self.mappers.iter().fold(seed, |acc, m| m.convert(acc))
    }
}

#[derive(Debug, PartialEq)]
struct Mapper {
    ranges: Vec<Range>,
}

impl Mapper {
    fn parse(input: &str) -> Self {
        let ranges = input.lines().filter_map(Range::try_parse).collect();

        Self { ranges }
    }

    fn convert(&self, source: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|r| r.convert(source))
            .unwrap_or(source)
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    dest: usize,
    source: usize,
    length: usize,
}

impl Range {
    fn try_parse(input: &str) -> Option<Self> {
        println!("{input}");

        let mut numbers = input.split_whitespace().flat_map(|n| n.parse());

        let dest = numbers.next()?;

        let source = numbers.next()?;

        let lenght = numbers.next()?;

        Some(Self {
            dest,
            source,
            length: lenght,
        })
    }

    fn convert(&self, input: usize) -> Option<usize> {
        if input >= self.source && input < self.source + self.length {
            Some(input - self.source + self.dest)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mapper_1() {
        let mapper = Range {
            dest: 50,
            source: 98,
            length: 2,
        };

        assert_eq!(mapper.convert(97), None);
        assert_eq!(mapper.convert(98), Some(50));
        assert_eq!(mapper.convert(99), Some(51));
        assert_eq!(mapper.convert(100), None);
    }

    #[test]
    fn example_mapper_2() {
        let mapper = Range {
            dest: 52,
            source: 50,
            length: 48,
        };

        assert_eq!(mapper.convert(49), None);
        assert_eq!(mapper.convert(50), Some(52));
        assert_eq!(mapper.convert(97), Some(99));
        assert_eq!(mapper.convert(98), None);
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

        let farmer = FarmerAlmanac::parse(include_str!("example"));

        assert_eq!(farmer, expected);
    }
}
