use crate::{parse, step, Point, Vec2};

pub fn restroom_redoubt(input: &str, grid: Point) {
    let mut robots = parse(input);

    for i in 1..10000 {
        step(&mut robots, grid);
        let output = debug(&robots, grid);

        if output.contains("#####################") {
            println!("{}", output);
            println!("{}", i);
            return;
        }
    }
}

fn debug(robots: &[(Point, Vec2)], grid: Point) -> String {
    let mut r = vec![vec!['.'; grid.1]; grid.0];

    for ((x, y), _) in robots {
        r[*x][*y] = '#'
    }

    let mut output = String::new();

    for y in 0..grid.1 {
        (0..grid.0).for_each(|x| {
            output.push(r[x][y]);
        });
        output.push('\n');
    }

    output
}
