pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> Matrix<usize> {
    let inner: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.bytes().map(|b| b as usize - 48).collect())
        .collect();

    let rows = inner.len();
    let cols = inner[0].len();

    Matrix { inner, rows, cols }
}

type Point = (usize, usize);
type PointI = (isize, isize);

#[derive(Debug)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    inner: Vec<Vec<T>>,
}

impl<T: PartialEq + Copy> Matrix<T> {
    fn get_coordinates(&self, predicate: fn(&T) -> bool) -> impl Iterator<Item = (Point, T)> + '_ {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(x, l)| l.iter().enumerate().map(move |(y, e)| (x, y, e)))
            .filter(move |(_, _, x)| predicate(x))
            .map(|(x, y, e)| ((x, y), *e))
    }

    fn get(&self, (x, y): Point) -> Option<T> {
        self.inner.get(x)?.get(y).copied()
    }

    fn valid_coordinates(&self, (x, y): PointI) -> Option<Point> {
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);

        if x >= self.rows || y >= self.cols {
            return None;
        }

        Some((x, y))
    }
}
