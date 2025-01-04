use std::collections::{HashMap, HashSet};

use crate::{parse, valid_point, Point};

pub fn resonant_collinearity(input: &str) -> usize {
    let (antennas, size) = parse(input);

    get_antinodes(&antennas, size).len()
}

fn get_antinodes(antennas: &HashMap<u8, Vec<(isize, isize)>>, size: isize) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for antennas in antennas.values() {
        for i in 0..antennas.len() {
            let left = antennas[i];

            for &right in &antennas[i + 1..] {
                let dx = right.0 - left.0;
                let dy = right.1 - left.1;

                let mut current = (left.0, left.1);

                while valid_point(current, size) {
                    antinodes.insert(current);
                    current = (current.0 - dx, current.1 - dy);
                }

                let mut current = (right.0, right.1);

                while valid_point(current, size) {
                    antinodes.insert(current);
                    current = (current.0 + dx, current.1 + dy);
                }
            }
        }
    }

    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let result = resonant_collinearity(input);
        assert_eq!(34, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = resonant_collinearity(input);
        assert_eq!(912, result);
    }
}
