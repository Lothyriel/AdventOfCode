use std::collections::HashMap;

pub mod part_1;
pub mod part_2;

type Point = (isize, isize);

fn valid_point(p: (isize, isize), size: isize) -> bool {
    p.0 >= 0 && p.0 < size && p.1 >= 0 && p.1 < size
}

fn parse(input: &str) -> (HashMap<u8, Vec<Point>>, isize) {
    let size = input.lines().next().expect("First").bytes().len();
    let mut antennas = HashMap::new();

    for (x, line) in input.lines().enumerate() {
        for (y, b) in line.bytes().enumerate() {
            match b {
                b'.' => continue,
                c => antennas
                    .entry(c)
                    .or_insert(vec![])
                    .push((x as isize, y as isize)),
            }
        }
    }

    (antennas, size as isize)
}
