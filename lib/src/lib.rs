pub type Point = (usize, usize);

#[derive(Debug)]
pub struct Matrix<T> {
    size: usize,
    inner: Vec<T>,
}

impl<T> Matrix<T> {
    const ADJACENT_OFFSETS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    const ORTHO_ADJACENT_OFFSETS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub fn parse<F: Fn(u8) -> T>(input: &str, builder: F) -> Matrix<T> {
        let size = input.lines().next().expect("First").len();

        let inner = input
            .lines()
            .flat_map(|l| l.bytes().map(&builder))
            .collect();

        Matrix { inner, size }
    }

    pub fn coords_filter(&self, predicate: fn(&T) -> bool) -> impl Iterator<Item = Point> {
        self.inner
            .iter()
            .enumerate()
            .filter(move |(_, x)| predicate(x))
            .map(|(i, _)| (i / self.size, i % self.size))
    }

    pub fn adjacent(&self, (x, y): (usize, usize)) -> impl Iterator<Item = &T> {
        Self::ADJACENT_OFFSETS
            .iter()
            .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter_map(|(dx, dy)| self.get_i((dx, dy)))
    }

    pub fn ortho_adjacent(&self, (x, y): (usize, usize)) -> impl Iterator<Item = &T> {
        Self::ORTHO_ADJACENT_OFFSETS
            .iter()
            .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter_map(|(dx, dy)| self.get_i((dx, dy)))
    }

    pub fn get(&self, (x, y): Point) -> Option<&T> {
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

        self.get((x as usize, y as usize))
    }

    pub fn set(&mut self, (x, y): Point, value: T) -> Option<T> {
        self.get((x, y))?;

        let old = std::mem::replace(&mut self.inner[x * self.size + y], value);

        Some(old)
    }
}
