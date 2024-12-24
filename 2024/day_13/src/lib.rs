pub mod part_1;
pub mod part_2;

struct Configuration {
    prize: Point,
    a: Point,
    b: Point,
}

impl Configuration {
    fn min_tokens(&self) -> Option<isize> {
        let d = det(self.a, self.b);
        let d_b = det(self.prize, self.b);
        let d_a = det(self.a, self.prize);

        if d_b % d == 0 && d_a % d == 0 {
            let min = 3 * d_b / d + d_a / d;
            Some(min)
        } else {
            None
        }
    }
}

fn det(a: Point, b: Point) -> isize {
    a.0 * b.1 - a.1 * b.0
}

type Point = (isize, isize);

fn parse(input: &str) -> Vec<Configuration> {
    input
        .split("\n\n")
        .map(|parts| {
            let mut parts = parts.lines();

            let a = parse_x_y(parts.next().expect("A"));
            let b = parse_x_y(parts.next().expect("B"));
            let prize = parse_x_y(parts.next().expect("Prize"));

            Configuration { a, b, prize }
        })
        .collect()
}

fn parse_x_y(input: &str) -> Point {
    let mut parts = input.split(": ").nth(1).expect("Numbers").split(", ");

    let mut parse = || parse_number(parts.next().expect("Part"));

    (parse(), parse())
}

fn parse_number(input: &str) -> isize {
    input
        .bytes()
        .filter(u8::is_ascii_digit)
        .rev()
        .enumerate()
        .map(|(i, d)| (d as isize - 48) * 10isize.pow(i as u32))
        .sum()
}
