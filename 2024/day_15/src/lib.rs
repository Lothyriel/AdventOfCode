use std::collections::VecDeque;

pub mod part_1;

pub fn parse(input: &str) -> (Warehouse, Vec<Direction>) {
    let mut parts = input.split("\n\n");

    let size = input.lines().next().expect("First line").bytes().len();

    let inner = parts
        .next()
        .expect("Grid")
        .lines()
        .flat_map(|l| {
            l.bytes().map(|b| match b {
                b'#' => Tile::Wall,
                b'@' => Tile::Robot,
                b'.' => Tile::Empty,
                b'O' => Tile::Box,
                _ => panic!("Invalid tile"),
            })
        })
        .collect();

    let grid = Matrix { inner, size };

    let robot = grid
        .get_coordinates(|t| *t == Tile::Robot)
        .next()
        .expect("Robot");

    let commands = parts
        .next()
        .expect("Commands")
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

    (Warehouse { robot, grid }, commands)
}

pub struct Warehouse {
    grid: Matrix<Tile>,
    robot: Point,
}
impl Warehouse {
    fn process_commands(&mut self, commands: &[Direction]) {
        for command in commands {
            self.process_command(command);
        }
    }

    pub fn gps_coordinates_count(&self) -> usize {
        self.grid
            .get_coordinates(|t| *t == Tile::Box)
            .map(|(x, y)| x * 100 + y)
            .sum()
    }

    pub fn process_command(&mut self, command: &Direction) {
        match self.valid_movement(self.robot, command) {
            Some(Movement::Push(pushes)) => {
                println!("Push {pushes:?}");
                self.push(pushes, command);
            }
            Some(Movement::Normal(c)) => {
                self.grid.set(self.robot, Tile::Empty);
                self.grid.set(c, Tile::Robot);
                self.robot = c;
            }
            None => {}
        }
    }

    fn push(&mut self, pushes: VecDeque<Point>, direction: &Direction) {
        for &push in pushes.iter() {
            self.grid.set(push, Tile::Empty);

            let push = new_pos_unchecked(direction, push).expect("Valid pos");

            self.grid.set(push, Tile::Box);
        }

        let next = *pushes.iter().last().expect("First push");

        self.grid.set(self.robot, Tile::Empty);
        self.grid.set(next, Tile::Robot);
        self.robot = next;
    }

    fn valid_movement(&self, from: Point, direction: &Direction) -> Option<Movement> {
        let new = new_pos_unchecked(direction, from)?;

        match self.grid.get(new)? {
            Tile::Wall => None,
            Tile::Box => self.valid_push(new, direction).map(Movement::Push),
            Tile::Empty => Some(Movement::Normal(new)),
            Tile::Robot => panic!("Shouldn't happen..."),
        }
    }

    fn valid_push(&self, from: Point, direction: &Direction) -> Option<VecDeque<(usize, usize)>> {
        let mut pushes = VecDeque::from([from]);

        match self.valid_movement(from, direction)? {
            Movement::Push(mut p) => {
                while let Some(p) = p.pop_back() {
                    pushes.push_front(p);
                }
                Some(pushes)
            }
            Movement::Normal(_) => Some(pushes),
        }
    }

    pub fn debug_grid(&self) -> String {
        let mut s = String::new();

        for x in 0..self.grid.size {
            for y in 0..self.grid.size {
                let ch = match self.grid.get((x, y)).expect("Valid") {
                    Tile::Wall => '#',
                    Tile::Box => 'O',
                    Tile::Empty => '.',
                    Tile::Robot => '@',
                };

                s.push(ch);
            }
            s.push('\n');
        }

        s
    }
}

fn new_pos_unchecked(direction: &Direction, (x, y): (usize, usize)) -> Option<Point> {
    let (dx, dy) = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };

    Some((x.checked_add_signed(dx)?, y.checked_add_signed(dy)?))
}

type Point = (usize, usize);

enum Movement {
    Push(VecDeque<Point>),
    Normal(Point),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Box,
    Empty,
    Robot,
}

#[derive(Debug)]
struct Matrix<T> {
    size: usize,
    inner: Vec<T>,
}

impl<T: PartialEq + Copy> Matrix<T> {
    fn get_coordinates(&self, predicate: fn(&T) -> bool) -> impl Iterator<Item = Point> + '_ {
        self.inner
            .iter()
            .enumerate()
            .filter(move |(_, x)| predicate(x))
            .map(|(i, _)| (i / self.size, i % self.size))
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        if x < self.size && y < self.size {
            self.inner.get(x * self.size + y)
        } else {
            None
        }
    }

    fn set(&mut self, (x, y): Point, value: T) -> Option<T> {
        let old = *self.get((x, y))?;

        self.inner[x * self.size + y] = value;

        Some(old)
    }
}
