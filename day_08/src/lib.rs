use std::collections::HashMap;

pub mod part_1;
pub mod part_2;

fn get_steps<'a>(
    directions: &[Direction],
    mut current: &'a str,
    nodes: &'a HashMap<&str, (&str, &str)>,
) -> usize {
    for (steps, direction) in directions.iter().cycle().enumerate() {
        if current == "ZZZ" {
            return steps;
        }

        let next = nodes.get(current).expect("Should have this node");

        current = match direction {
            Direction::Left => next.0,
            Direction::Right => next.1,
        };
    }

    unreachable!("Cycled iteration shouldn't finish")
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
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
        .collect();

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
