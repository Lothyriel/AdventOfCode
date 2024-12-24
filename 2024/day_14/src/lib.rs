use std::str::FromStr;

pub mod part_1;
pub mod part_2;

fn simulate(robots: &mut [(Point, Vec2)], grid: Point, seconds: usize) {
    for _ in 0..seconds {
        step(robots, grid);
    }
}

fn step(robots: &mut [(Point, Vec2)], grid: (usize, usize)) {
    for (pos, vel) in robots.iter_mut() {
        let new_x = (pos.0 as isize + vel.0).rem_euclid(grid.0 as isize);
        let new_y = (pos.1 as isize + vel.1).rem_euclid(grid.1 as isize);

        *pos = (new_x as usize, new_y as usize);
    }
}

type Vec2 = (isize, isize);
type Point = (usize, usize);

fn parse(input: &str) -> Vec<(Point, Vec2)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');

            let pos = parse_vec2(parts.next().expect("Pos"));
            let vel = parse_vec2(parts.next().expect("Vel"));

            (pos, vel)
        })
        .collect()
}

fn parse_vec2<T: FromStr>(input: &str) -> (T, T)
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut parts = input[2..].split(',');

    let mut parse = || parts.next().expect("Part").parse().expect("Number");

    (parse(), parse())
}
