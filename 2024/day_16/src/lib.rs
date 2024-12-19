pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> MazeRunner {
    let size = input.lines().next().expect("First").bytes().len();

    let inner = input
        .lines()
        .flat_map(|l| {
            l.bytes().map(|b| match b {
                b'#' => Tile::Wall,
                b'S' => Tile::Start,
                b'E' => Tile::Finish,
                b'.' => Tile::Empty,
                _ => panic!("Invalid tile"),
            })
        })
        .collect();

    let maze = Matrix { inner, size };

    let start = maze
        .get_coordinates(|x| *x == Tile::Start)
        .next()
        .expect("Start");

    MazeRunner {
        position: start,
        direction: Direction::East,
        maze,
    }
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    point: Point,
    cost: usize,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Point {
    coordinates: (usize, usize),
    direction: Direction,
}

impl Point {
    fn new_coordinates(&self, coordinates: (usize, usize)) -> Point {
        Point {
            direction: self.direction,
            coordinates,
        }
    }

    fn new_direction(&self, direction: Direction) -> Point {
        Point {
            direction,
            coordinates: self.coordinates,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Start,
    Finish,
    Wall,
    Empty,
}

#[derive(Debug)]
struct MazeRunner {
    maze: Matrix<Tile>,
    position: (usize, usize),
    direction: Direction,
}

impl MazeRunner {
    fn get_forward_coordinates(&self, state: &State) -> Option<(usize, usize)> {
        let (dx, dy) = match state.point.direction {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        };

        let (x, y) = state.point.coordinates;

        let new_coordinates = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);

        if self.maze.get(new_coordinates)? != &Tile::Wall {
            Some(new_coordinates)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Matrix<T> {
    size: usize,
    inner: Vec<T>,
}

impl<T: PartialEq> Matrix<T> {
    fn get_coordinates(
        &self,
        predicate: fn(&T) -> bool,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
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
}
