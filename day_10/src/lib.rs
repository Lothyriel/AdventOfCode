use std::collections::HashSet;

pub mod part_1;

struct PipeMaze {
    tiles: Vec<Tile>,
    size: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
    Start,
    Ground,
}

const OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

impl PipeMaze {
    fn farthest(&self) -> usize {
        let (x, y) = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, t)| matches!(t, Tile::Start))
            .map(|(i, _)| (i % self.size, i / self.size))
            .expect("Should have start tile");

        let mut heads: Vec<_> = self.neighbor_pipes(x, y).collect();

        let mut visited: HashSet<_> = HashSet::from_iter(vec![(x, y)]);

        let mut count = 0;

        loop {
            let mut new_nodes = Vec::new();
            println!("Visited: {visited:?}");

            for &n in heads.iter() {
                if visited.insert((n.1, n.2)) {
                    new_nodes.append(&mut self.neighbor_pipes(n.1, n.2).collect());
                }
            }

            if new_nodes.is_empty() {
                break count;
            }

            count += 1;
            heads = new_nodes;
        }
    }

    fn at(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(x + (y * self.size)).copied()
    }

    fn neighbor_pipes(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (Tile, usize, usize)> + '_ {
        self.neighbors(x, y)
            .filter(|t| !matches!(t.0, Tile::Ground))
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (Tile, usize, usize)> + '_ {
        OFFSETS.iter().filter_map(move |(x1, y1)| {
            let x = x.checked_add_signed(*x1 as isize)?;
            let y = y.checked_add_signed(*y1 as isize)?;

            self.at(x, y).map(|n| (n, x, y))
        })
    }

    fn parse(input: &str) -> Self {
        let size = input.lines().count();

        let tiles = input
            .lines()
            .flat_map(|line| {
                line.bytes().map(|t| match t {
                    b'|' => Tile::Vertical,
                    b'-' => Tile::Horizontal,
                    b'L' => Tile::DownRight,
                    b'J' => Tile::DownLeft,
                    b'7' => Tile::UpLeft,
                    b'F' => Tile::UpRight,
                    b'.' => Tile::Ground,
                    b'S' => Tile::Start,
                    c => panic!("Invalid character: {}", c as char),
                })
            })
            .collect();

        Self { tiles, size }
    }
}
