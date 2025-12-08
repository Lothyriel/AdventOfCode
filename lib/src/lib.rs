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

    pub fn down(&self, (x, y): Point) -> Option<&T> {
        self.get_i((x as isize + 1, y as isize))
    }
}

pub struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl Dsu {
    pub fn new(count: usize) -> Self {
        Self {
            parent: (0..count).collect(),
            size: vec![1; count],
            count,
        }
    }

    /// Returns the parent node id/index that contains `x`.
    /// Performs path compression to optimize future calls.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
        }

        self.parent[x]
    }

    /// Merges the sets containing `a` and `b`.
    /// Returns `true` if a merge happened, or `false` if
    /// `a` and `b` were already in the same set.
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if a == b {
            return false;
        }

        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.count -= 1;

        true
    }

    /// Returns the sorted (DSC) list of components sizes.
    pub fn components(&mut self) -> Vec<usize> {
        let mut components = Vec::new();

        for i in 0..self.parent.len() {
            if self.find(i) == i {
                components.push(self.size[i]);
            }
        }

        components.sort_unstable_by(|a, b| b.cmp(a));
        components
    }

    /// Returns the current number of components.
    pub fn count(&self) -> usize {
        self.count
    }
}
