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

                let d_right = (right.0 + dx, right.1 + dy);
                let d_left = (left.0 - dx, left.1 - dy);

                for point in [d_left, d_right] {
                    if valid_point(point, size) {
                        antinodes.insert(point);
                    }
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
        assert_eq!(14, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = resonant_collinearity(input);
        assert_eq!(244, result);
    }
}
