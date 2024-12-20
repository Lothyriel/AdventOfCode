use std::collections::HashMap;

pub mod part_1;
pub mod part_2;

pub fn cheating_shortcuts(input: &str, cheat_time: isize) -> HashMap<usize, usize> {
    let track = parse(input);

    let end = track
        .get_coordinates(|t| *t == Tile::End)
        .next()
        .expect("End");

    get_cheat_paths_count(&track, end, cheat_time)
}

fn get_cheat_paths_count(
    track: &Matrix<Tile>,
    end: Point,
    cheat_time: isize,
) -> HashMap<usize, usize> {
    let distances = get_distance_matrix(track, end);

    let mut cheat_paths = HashMap::new();

    for (x, y) in track.get_coordinates(|_| true) {
        if distances.get((x, y)) == Some(&usize::MAX) {
            continue;
        }

        let (x, y) = (x as isize, y as isize);

        for dx in x - cheat_time..=x + cheat_time {
            for dy in y - cheat_time..=y + cheat_time {
                if let Some(saved) = get_saved_distance(&distances, (x, y), (dx, dy), cheat_time) {
                    *cheat_paths.entry(saved).or_insert(0) += 1;
                }
            }
        }
    }

    cheat_paths
}

fn get_saved_distance(
    distance_matrix: &Matrix<usize>,
    start: (isize, isize),
    end: (isize, isize),
    cheat_time: isize,
) -> Option<usize> {
    let start_d = distance_matrix.get_i(start)?;
    let end_d = distance_matrix.get_i(end)?;

    let cheating = manhattan_distance(start, end);

    if cheating > cheat_time as usize {
        return None;
    }

    let normal = start_d.checked_sub(*end_d)?;

    Some(normal - cheating)
}

fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn get_distance_matrix(track: &Matrix<Tile>, to: (usize, usize)) -> Matrix<usize> {
    let mut matrix = Matrix {
        size: track.size,
        inner: vec![usize::MAX; track.size * track.size],
    };

    matrix.set(to, 0);

    let mut to_explore = vec![(to, 0)];

    while !to_explore.is_empty() {
        to_explore = explore_neighbours(track, &mut matrix, &to_explore);
    }

    matrix
}

fn explore_neighbours(
    track: &Matrix<Tile>,
    distance_matrix: &mut Matrix<usize>,
    to_explore: &Vec<((usize, usize), usize)>,
) -> Vec<(Point, usize)> {
    let mut next = vec![];

    for (pos, distance) in to_explore {
        for direction in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            if let Some(new) = valid_neighbour(track, *pos, direction) {
                let new_distance = distance + 1;

                let current_distance = distance_matrix.get(new).expect("Should be valid");

                if new_distance >= *current_distance {
                    continue;
                }

                distance_matrix.set(new, new_distance);
                next.push((new, new_distance));
            };
        }
    }

    next
}

fn valid_neighbour(track: &Matrix<Tile>, (x, y): Point, (dx, dy): (isize, isize)) -> Option<Point> {
    let new = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);

    if track.get(new)? != &Tile::Wall {
        Some(new)
    } else {
        None
    }
}

fn parse(input: &str) -> Matrix<Tile> {
    let size = input.lines().next().expect("First").bytes().len();

    let inner = input
        .lines()
        .flat_map(|l| {
            l.bytes().map(|b| match b {
                b'#' => Tile::Wall,
                b'S' => Tile::Start,
                b'E' => Tile::End,
                b'.' => Tile::Empty,
                _ => panic!("Invalid tile"),
            })
        })
        .collect();

    Matrix { inner, size }
}

type Point = (usize, usize);

#[derive(Debug)]
pub struct Matrix<T> {
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

    fn get(&self, (x, y): Point) -> Option<&T> {
        if x < self.size && y < self.size {
            self.inner.get(x * self.size + y)
        } else {
            None
        }
    }

    fn get_i(&self, (x, y): (isize, isize)) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }

        if x < self.size as isize && y < self.size as isize {
            self.inner.get(x as usize * self.size + y as usize)
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Start,
    End,
    Empty,
}
