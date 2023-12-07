pub mod part_1;
pub mod part_2;

pub static mut COMPARISONS: usize = 0;
pub static mut CALCULATIONS: usize = 0;

#[derive(Debug, PartialEq)]
pub struct FarmerAlmanac {
    mappers: Vec<Mapper>,
    pub seeds: Vec<usize>,
}

impl FarmerAlmanac {
    pub fn get_lowest_location(&self) -> usize {
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
        unsafe {
            COMPARISONS += 1;
        }
        if input >= self.source && input < self.source + self.length {
            unsafe {
                CALCULATIONS += 1;
            }
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
}
