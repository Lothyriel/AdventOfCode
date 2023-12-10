use std::collections::HashMap;

pub mod part_1;

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = Direction> + '_,
    HashMap<&str, (&str, &str)>,
) {
    let mut lines = input.lines();

    let directions = lines
        .next()
        .expect("Should contain directions")
        .bytes()
        .flat_map(|d| match d {
            b'L' => Some(Direction::Left),
            b'R' => Some(Direction::Right),
            _ => None,
        })
        .cycle();

    let _empty_line = lines.next().expect("Should consume empty line");

    let nodes: HashMap<_, _> = lines.map(parse_line).collect();

    (directions, nodes)
}

fn parse_line(input: &str) -> (&str, (&str, &str)) {
    let mut parts = input.split('=');

    let node = parts.next().expect("Should contain node");

    let nodes = parts.next().expect("Should contain nodes");

    (node.trim(), parse_nodes(nodes))
}

fn parse_nodes(input: &str) -> (&str, &str) {
    let mut nodes = input.split(',');

    const TRIM: &[char] = &['(', ')', ' '];

    let mut g = || {
        nodes
            .next()
            .expect("Should contain nodes")
            .trim_matches(TRIM)
    };

    (g(), g())
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}
