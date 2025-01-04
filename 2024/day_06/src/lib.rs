use std::collections::HashSet;

pub mod part_1;
pub mod part_2;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Obstacle,
    Empty,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Map {
    inner: Matrix<Tile>,
    guard_pos: Point,
    guard_dir: Direction,
}

impl Map {
    fn parse(input: &str) -> Self {
        let size = input.lines().next().expect("First").bytes().len();

        let inner = Matrix {
            size,
            inner: input.lines().flat_map(|line| line.bytes()).collect(),
        };

        let guard_pos = inner.get_coordinates(|t| *t == b'^').next().expect("Guard");

        let inner = inner.map(|b| match b {
            b'^' | b'.' => Tile::Empty,
            b'#' => Tile::Obstacle,
            t => panic!("Invalid tile {}", t as char),
        });

        Self {
            inner,
            guard_pos,
            guard_dir: Direction::Up,
        }
    }

    fn get_path(&mut self) -> HashSet<Point> {
        let mut path = HashSet::from([self.guard_pos]);

        while let Some((pos, dir)) = self.next_patroll() {
            self.guard_pos = pos;
            self.guard_dir = dir;
            path.insert(pos);
        }

        path
    }

    fn next_patroll(&self) -> Option<(Point, Direction)> {
        let next_pos = match self.guard_dir {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        let next_pos = (
            self.guard_pos.0 as isize + next_pos.0,
            self.guard_pos.1 as isize + next_pos.1,
        );

        let next = (next_pos.0 as usize, next_pos.1 as usize);

        let next = match self.inner.get(next_pos)? {
            Tile::Obstacle => (self.guard_pos, self.guard_dir.turn_right()),
            Tile::Empty => (next, self.guard_dir),
        };

        Some(next)
    }

    fn patroll_loops(&mut self) -> bool {
        let mut turns = HashSet::from([(self.guard_pos, self.guard_dir)]);

        while let Some((pos, dir)) = self.next_patroll() {
            self.guard_pos = pos;
            self.guard_dir = dir;

            if !turns.insert((pos, dir)) {
                return true;
            }
        }

        false
    }
}

type Point = (usize, usize);

pub struct Matrix<T> {
    size: usize,
    inner: Vec<T>,
}

impl<T: PartialEq + Copy> Matrix<T> {
    fn map<R: PartialEq + Copy>(self, mapper: fn(T) -> R) -> Matrix<R> {
        let inner = self.inner.into_iter().map(mapper).collect();

        Matrix {
            size: self.size,
            inner,
        }
    }

    fn get_coordinates(&self, predicate: fn(&T) -> bool) -> impl Iterator<Item = Point> + '_ {
        self.inner
            .iter()
            .enumerate()
            .filter(move |(_, x)| predicate(x))
            .map(|(i, _)| (i / self.size, i % self.size))
    }

    fn get(&self, (x, y): (isize, isize)) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }

        if x < self.size as isize && y < self.size as isize {
            self.inner.get(x as usize * self.size + y as usize)
        } else {
            None
        }
    }

    fn set(&mut self, (x, y): Point, value: T) {
        self.inner[x * self.size + y] = value;
    }
}
