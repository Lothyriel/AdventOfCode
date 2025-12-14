use lib::Point;

use crate::{Rectangle, parse, rectangles};

pub fn movie_theater(input: &str) -> usize {
    let points = parse(input);

    rectangles(&points)
        .filter(|r| is_inside(r, &points))
        .map(|r| r.area())
        .max()
        .unwrap()
}

fn is_inside(r: &Rectangle, points: &[Point]) -> bool {
    let (xmin, xmax, ymin, ymax) = r.edges();

    for (i, p1) in points.iter().enumerate() {
        let p2 = &points[(i + 1) % points.len()];

        let crosses_horizontally = || {
            if p1.1 == p2.1 {
                let (xlmin, xlmax) = (p1.0.min(p2.0), p1.0.max(p2.0));
                ymin < p1.1 && ymax > p1.1 && !(xmin >= xlmax || xmax <= xlmin)
            } else {
                false
            }
        };

        let crosses_vertically = || {
            if p1.0 == p2.0 {
                let (ylmin, ylmax) = (p1.1.min(p2.1), p1.1.max(p2.1));
                xmin < p1.0 && xmax > p1.0 && !(ymin >= ylmax || ymax <= ylmin)
            } else {
                false
            }
        };

        if crosses_vertically() || crosses_horizontally() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

        let result = movie_theater(input);
        assert_eq!(24, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = movie_theater(input);
        assert_eq!(1573359081, result);
    }
}
