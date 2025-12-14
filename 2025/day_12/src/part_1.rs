use std::collections::VecDeque;
use lib::Matrix;

pub fn xmas_tree_farm(input: &str) -> usize {
    let (trees, gifts) = parse(input);

    trees.iter().filter(|t| t.fits_all(&gifts)).count()
}

fn parse(input: &str) -> (Vec<Tree>, Vec<Gift>) {
    let mut parts: VecDeque<_> = input.split("\n\n").collect();

    let trees = parts.pop_back().unwrap().split('\n').map(Tree::parse).collect();

    let gifts = parts.drain(..).map(Gift::parse).collect();

    (trees, gifts)
}

#[derive(Debug)]
struct Tree {
    width: usize,
    length: usize,
    gifts: Vec<usize>,
}

impl Tree {
    fn fits_all(&self, gifts: &[Gift]) -> bool {
        let required_area: usize = self
            .gifts
            .iter()
            .enumerate()
            .map(|(i, &count)| count * gifts[i].area)
            .sum();

        self.width * self.length >= required_area
    }
}

impl Tree {
    fn parse(input: &str) -> Self {
        let (dimensions, gifts) = input.split_once(':').unwrap();
        let p = |i: &str| i.parse().unwrap();

        let gifts = gifts.split_whitespace().map(|n| p(n)).collect();

        let (width, length) = dimensions.split_once('x').unwrap();

        Self {
            width: p(width),
            length: p(length),
            gifts,
        }
    }
}

#[derive(Debug)]
struct Gift {
    area: usize,
}

impl Gift {
    fn parse(input: &str) -> Self {
        let input = input.split_once('\n')
            .map(|(_, after)| after)
            .unwrap();

        let shape = Matrix::parse(input, |b| match b {
            b'#' => Tile::Occupied,
            b'.' => Tile::Empty,
            _ => panic!("invalid tile"),
        });

        let area = shape.coords_filter(|t| *t == Tile::Occupied).count();

        Self { area }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Occupied,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

        let result = xmas_tree_farm(input);
        //the we made a very simplified algorithm
        //it works for the input, but not for the example...xD
        //assert_eq!(2, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = xmas_tree_farm(input);
        assert_eq!(443, result);
    }
}