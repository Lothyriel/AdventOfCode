use std::collections::VecDeque;

pub mod part_1;
pub mod part_2;

fn parse_commands(input: &str) -> Vec<Direction> {
    let commands = input
        .lines()
        .flat_map(|l| {
            l.bytes().map(|b| match b {
                b'>' => Direction::Right,
                b'<' => Direction::Left,
                b'v' => Direction::Down,
                b'^' => Direction::Up,
                _ => panic!("Invalid command"),
            })
        })
        .collect();
    commands
}

fn new_pos_unchecked(direction: &Direction, (x, y): Point) -> Option<Point> {
    let (dx, dy) = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };

    Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?))
}

type Point = (usize, usize);

#[derive(Debug)]
enum Movement {
    Push(VecDeque<Point>),
    Normal(Point),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    inner: Vec<T>,
}

impl<T: PartialEq + Copy> Matrix<T> {
    fn get_coordinates(&self, predicate: fn(&T) -> bool) -> impl Iterator<Item = Point> + '_ {
        self.inner
            .iter()
            .enumerate()
            .filter(move |(_, x)| predicate(x))
            .map(|(i, _)| (i / self.cols, i % self.cols))
    }

    fn get(&self, (x, y): Point) -> Option<&T> {
        if x < self.rows && y < self.cols {
            self.inner.get(x * self.cols + y)
        } else {
            None
        }
    }

    fn set(&mut self, (x, y): Point, value: T) -> Option<T> {
        let old = *self.get((x, y))?;

        self.inner[x * self.cols + y] = value;

        Some(old)
    }
}
