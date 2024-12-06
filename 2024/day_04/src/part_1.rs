pub fn ceres_search(input: &str) -> usize {
    let matrix = parse(input);

    let scan_patterns = [
        Matrix::horizontal_forwards,
        Matrix::horizontal_backwards,
        Matrix::vertical_upwards,
        Matrix::vertical_downwards,
        Matrix::diagonal_ne,
        Matrix::diagonal_nw,
        Matrix::diagonal_se,
        Matrix::diagonal_sw,
    ];

    matrix.xmas_count(&scan_patterns)
}

fn parse(input: &str) -> Matrix {
    let line_size = input.lines().next().expect("First").bytes().len();

    if input.lines().any(|l| l.bytes().len() != line_size) {
        panic!("Expected all lines to have the same length");
    }

    let inner = input
        .lines()
        .flat_map(|l| l.bytes())
        .map(|l| match l {
            b'X' => Letter::X,
            b'M' => Letter::M,
            b'A' => Letter::A,
            b'S' => Letter::S,
            _ => panic!("Unexpected token"),
        })
        .collect();

    Matrix { line_size, inner }
}

struct Matrix {
    inner: Vec<Letter>,
    line_size: usize,
}

type ScanPattern = fn(&Matrix, (usize, usize)) -> Option<()>;

impl Matrix {
    fn xmas_count(&self, scan_patterns: &[ScanPattern]) -> usize {
        self.x_indexes()
            .map(|c| scan_patterns.iter().filter_map(|p| p(self, c)).count())
            .sum()
    }

    fn x_indexes(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.inner
            .iter()
            .enumerate()
            .filter(|(_, l)| matches!(l, Letter::X))
            .map(|(i, _)| (i / self.line_size, i % self.line_size))
    }

    fn horizontal_forwards(&self, c: (usize, usize)) -> Option<()> {
        self.check(c, (0, 1))
    }

    fn horizontal_backwards(&self, c: (usize, usize)) -> Option<()> {
        self.check(c, (0, -1))
    }

    fn vertical_downwards(&self, c: (usize, usize)) -> Option<()> {
        self.check(c, (1, 0))
    }

    fn vertical_upwards(&self, c: (usize, usize)) -> Option<()> {
        self.check(c, (-1, 0))
    }

    fn diagonal_ne(&self, c: (usize, usize)) -> Option<()> {
        self.check(c, (-1, 1))
    }

    fn diagonal_nw(&self, c: (usize, usize)) -> Option<()> {
        self.check(c, (-1, -1))
    }

    fn diagonal_se(&self, c: (usize, usize)) -> Option<()> {
        self.check(c, (1, 1))
    }

    fn diagonal_sw(&self, c: (usize, usize)) -> Option<()> {
        self.check(c, (1, -1))
    }

    fn check(&self, c: (usize, usize), pattern: (isize, isize)) -> Option<()> {
        let mul = |n| (pattern.0 * n, pattern.1 * n);

        self.matches(c, (0, 0), Letter::X)?;
        self.matches(c, mul(1), Letter::M)?;
        self.matches(c, mul(2), Letter::A)?;
        self.matches(c, mul(3), Letter::S)?;

        Some(())
    }

    fn matches(
        &self,
        (x, y): (usize, usize),
        pattern: (isize, isize),
        letter: Letter,
    ) -> Option<()> {
        let (x, y) = (
            x.checked_add_signed(pattern.0)?,
            y.checked_add_signed(pattern.1)?,
        );

        if y >= self.line_size || x >= self.line_size {
            return None;
        }

        let matching = self.inner.get(x * self.line_size + y)?;

        if *matching == letter {
            Some(())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let matrix = parse(input);

        assert_eq!(3, matrix.xmas_count(&[Matrix::horizontal_forwards]));
        assert_eq!(2, matrix.xmas_count(&[Matrix::horizontal_backwards]));
        assert_eq!(1, matrix.xmas_count(&[Matrix::vertical_downwards]));
        assert_eq!(2, matrix.xmas_count(&[Matrix::vertical_upwards]));
        assert_eq!(4, matrix.xmas_count(&[Matrix::diagonal_ne]));
        assert_eq!(4, matrix.xmas_count(&[Matrix::diagonal_nw]));
        assert_eq!(1, matrix.xmas_count(&[Matrix::diagonal_se]));
        assert_eq!(1, matrix.xmas_count(&[Matrix::diagonal_sw]));

        assert_eq!(18, ceres_search(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(2462, ceres_search(input));
    }
}
