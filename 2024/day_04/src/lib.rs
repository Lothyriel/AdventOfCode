pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> Matrix {
    let line_size = input.lines().next().expect("First").bytes().len();

    if input.lines().any(|l| l.bytes().len() != line_size) {
        panic!("Expected all lines to have the same length");
    }

    let inner = input
        .lines()
        .flat_map(|l| l.bytes())
        .map(|l| match l {
            b'X' => Symbol::X,
            b'M' => Symbol::M,
            b'A' => Symbol::A,
            b'S' => Symbol::S,
            b'.' => Symbol::Dot,
            _ => panic!("Unexpected symbol"),
        })
        .collect();

    Matrix { line_size, inner }
}

struct Matrix {
    inner: Vec<Symbol>,
    line_size: usize,
}

type ScanPattern = fn(&Matrix, (usize, usize)) -> Option<()>;

impl Matrix {
    fn x_mas_count(&self) -> usize {
        self.filter_indexes(Symbol::A)
            .filter_map(|c| self.x_mas(c))
            .count()
    }

    fn x_mas(&self, c: (usize, usize)) -> Option<()> {
        self.x_mas_left(c)
            .or(self.x_mas_top(c))
            .or(self.x_mas_right(c))
            .or(self.x_mas_bottom(c))
    }

    fn x_mas_left(&self, c: (usize, usize)) -> Option<()> {
        self.matches(c, (-1, -1), Symbol::M)?;
        self.matches(c, (-1, 1), Symbol::S)?;
        self.matches(c, (1, -1), Symbol::M)?;
        self.matches(c, (1, 1), Symbol::S)?;

        Some(())
    }

    fn x_mas_top(&self, c: (usize, usize)) -> Option<()> {
        self.matches(c, (-1, -1), Symbol::M)?;
        self.matches(c, (-1, 1), Symbol::M)?;
        self.matches(c, (1, -1), Symbol::S)?;
        self.matches(c, (1, 1), Symbol::S)?;

        Some(())
    }

    fn x_mas_right(&self, c: (usize, usize)) -> Option<()> {
        self.matches(c, (-1, -1), Symbol::S)?;
        self.matches(c, (-1, 1), Symbol::M)?;
        self.matches(c, (1, -1), Symbol::S)?;
        self.matches(c, (1, 1), Symbol::M)?;

        Some(())
    }

    fn x_mas_bottom(&self, c: (usize, usize)) -> Option<()> {
        self.matches(c, (-1, -1), Symbol::S)?;
        self.matches(c, (-1, 1), Symbol::S)?;
        self.matches(c, (1, -1), Symbol::M)?;
        self.matches(c, (1, 1), Symbol::M)?;

        Some(())
    }

    fn xmas_count(&self, scan_patterns: &[ScanPattern]) -> usize {
        self.filter_indexes(Symbol::X)
            .map(|c| scan_patterns.iter().filter_map(|p| p(self, c)).count())
            .sum()
    }

    fn filter_indexes(&self, letter: Symbol) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.inner
            .iter()
            .enumerate()
            .filter(move |(_, l)| **l == letter)
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

        self.matches(c, (0, 0), Symbol::X)?;
        self.matches(c, mul(1), Symbol::M)?;
        self.matches(c, mul(2), Symbol::A)?;
        self.matches(c, mul(3), Symbol::S)?;

        Some(())
    }

    fn matches(
        &self,
        (x, y): (usize, usize),
        pattern: (isize, isize),
        letter: Symbol,
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
enum Symbol {
    X,
    M,
    A,
    S,
    Dot,
}
