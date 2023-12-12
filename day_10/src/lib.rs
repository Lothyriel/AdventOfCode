use std::{collections::HashSet, time::Duration};

pub mod part_1;

struct PipeMaze {
    tiles: Vec<Tile>,
    size: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    V,
    H,
    L,
    J,
    F,
    Seven,
    Start,
    Ground,
}

type Offset = (isize, isize);

const UP: Offset = (-1, 0);
const LEFT: Offset = (0, -1);
const RIGHT: Offset = (0, 1);
const DOWN: Offset = (1, 0);
const OFFSETS: [Offset; 4] = [UP, LEFT, RIGHT, DOWN];

type TilePos = (Tile, usize, usize);

impl PipeMaze {
    fn farthest(&self, debug: bool) -> usize {
        let (x, y) = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, t)| matches!(t, Tile::Start))
            .map(|(i, _)| (i / self.size, i % self.size))
            .expect("Should have start tile");

        let mut nodes = vec![(Tile::Start, x, y)];

        let mut visited = HashSet::new();

        let mut count = 0;

        loop {
            let mut new = Vec::new();

            for &n in nodes.iter() {
                if visited.insert((n.1, n.2)) {
                    new.append(&mut self.explore(n, &visited));
                }
            }

            if debug {
                self.debug(&visited);
            }

            if new.is_empty() {
                break count;
            }

            count += 1;
            nodes = new;
        }
    }

    fn at(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get((x * self.size) + y).copied()
    }

    fn explore(&self, c: TilePos, v: &HashSet<(usize, usize)>) -> Vec<(Tile, usize, usize)> {
        self.neighbor_pipes(c.1, c.2)
            .filter(|n| !v.contains(&(n.1, n.2)))
            .filter(|&n| fits(c, n))
            .collect()
    }

    fn neighbor_pipes(&self, x: usize, y: usize) -> impl Iterator<Item = TilePos> + '_ {
        self.neighbors(x, y)
            .filter(|t| !matches!(t.0, Tile::Ground))
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (Tile, usize, usize)> + '_ {
        OFFSETS.iter().filter_map(move |(x1, y1)| {
            let x = x.checked_add_signed(*x1)?;
            let y = y.checked_add_signed(*y1)?;

            self.at(x, y).map(|n| (n, x, y))
        })
    }

    fn parse(input: &str) -> Self {
        let size = input.lines().count();

        let tiles = input
            .lines()
            .flat_map(|line| {
                line.bytes().map(|t| match t {
                    b'|' => Tile::V,
                    b'-' => Tile::H,
                    b'L' => Tile::L,
                    b'J' => Tile::J,
                    b'7' => Tile::Seven,
                    b'F' => Tile::F,
                    b'.' => Tile::Ground,
                    b'S' => Tile::Start,
                    c => panic!("Invalid character: {}", c as char),
                })
            })
            .collect();

        Self { tiles, size }
    }

    fn debug(&self, visited: &HashSet<(usize, usize)>) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for (x, l) in self.tiles.chunks_exact(self.size).enumerate() {
            let line: String = l
                .iter()
                .enumerate()
                .map(|(y, t)| match visited.contains(&(x, y)) {
                    false => ' ',
                    true => match t {
                        Tile::V => '│',
                        Tile::H => '━',
                        Tile::L => '└',
                        Tile::J => '┘',
                        Tile::F => '┏',
                        Tile::Seven => '┓',
                        Tile::Start => 'S',
                        Tile::Ground => '.',
                    },
                })
                .collect();

            println!("{line}");
        }
    }
}

fn fits(a: TilePos, b: TilePos) -> bool {
    let (x1, y1) = (a.1 as isize, a.2 as isize);
    let (x2, y2) = (b.1 as isize, b.2 as isize);
    let offset = (x2 - x1, y2 - y1);

    let c = |offsets: &[Offset]| offsets.iter().any(|&o| o == offset) && check_fit(offset, b.0);

    match a.0 {
        Tile::V => c(&[UP, DOWN]),
        Tile::H => c(&[LEFT, RIGHT]),
        Tile::L => c(&[UP, RIGHT]),
        Tile::J => c(&[UP, LEFT]),
        Tile::F => c(&[RIGHT, DOWN]),
        Tile::Seven => c(&[LEFT, DOWN]),
        Tile::Start => true,
        Tile::Ground => false,
    }
}

fn check_fit(offset: (isize, isize), other: Tile) -> bool {
    match offset {
        UP => matches!(other, Tile::V | Tile::F | Tile::Seven),
        DOWN => matches!(other, Tile::V | Tile::L | Tile::J),
        LEFT => matches!(other, Tile::H | Tile::F | Tile::L),
        RIGHT => matches!(other, Tile::H | Tile::Seven | Tile::J),
        _ => panic!("Invalid offset"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fits() {
        assert!(fits((Tile::V, 1, 2), (Tile::L, 2, 2)));
        assert!(!fits((Tile::V, 1, 2), (Tile::L, 1, 3)));
    }
}
