use itertools::Itertools;

pub mod part_1;
pub mod part_2;

fn solve(input: &str, factor: usize) -> isize {
    let (size, mut stars) = crate::parse(input);

    crate::expand_universe(&mut stars, size, factor);

    get_distances(stars)
}

fn get_distances(stars: Vec<(usize, usize)>) -> isize {
    stars
        .iter()
        .combinations(2)
        .map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            let (x1, y1) = (a.0 as isize, a.1 as isize);
            let (x2, y2) = (b.0 as isize, b.1 as isize);

            (x2 - x1).abs() + (y2 - y1).abs()
        })
        .sum()
}

fn expand_universe(stars: &mut [(usize, usize)], size: usize, factor: usize) {
    let factor = if factor > 1 { factor - 1 } else { factor };

    let empty_lines: Vec<_> = (0..size)
        .filter(|&x| !stars.iter().any(|s| s.0 == x))
        .collect();

    let empty_cols: Vec<_> = (0..size)
        .filter(|&y| !stars.iter().any(|s| s.1 == y))
        .collect();

    for star in stars.iter_mut() {
        let up = empty_lines.iter().filter(|&&c| star.0 > c).count() * factor;
        star.0 += up;

        let left = empty_cols.iter().filter(|&&c| star.1 > c).count() * factor;
        star.1 += left;
    }
}

fn parse(input: &str) -> (usize, Vec<(usize, usize)>) {
    let size = input.lines().count();

    let stars = input
        .lines()
        .enumerate()
        .flat_map(|(x, l)| parse_stars(x, l))
        .collect();

    (size, stars)
}

fn parse_stars(x: usize, line: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    line.bytes().enumerate().flat_map(move |(y, b)| match b {
        b'#' => Some((x, y)),
        _ => None,
    })
}
