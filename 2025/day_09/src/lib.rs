use lib::Point;

pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            let p = |i: &str| i.parse().unwrap();
            (p(x), p(y))
        })
        .collect()
}

fn rectangles(points: &[Point]) -> impl Iterator<Item = Rectangle> {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| points[i + 1..].iter().map(move |&p2| Rectangle(p1, p2)))
}

struct Rectangle(Point, Point);

impl Rectangle {
    fn area(&self) -> usize {
        let (p1, p2) = (self.0, self.1);
        (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
    }

    fn edges(&self) -> (usize, usize, usize, usize) {
        let (p1, p2) = (self.0, self.1);

        (
            p1.0.min(p2.0),
            p1.0.max(p2.0),
            p1.1.min(p2.1),
            p1.1.max(p2.1),
        )
    }
}
