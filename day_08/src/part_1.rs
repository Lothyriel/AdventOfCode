use std::collections::HashMap;

pub fn get_steps_count(input: &str) -> usize {
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

    let mut current = "AAA";

    for (steps, direction) in directions.enumerate() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_steps_count(include_str!("example"));
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = get_steps_count(include_str!("example_2"));
        assert_eq!(result, 6);
    }

    #[test]
    fn puzzle() {
        let result = get_steps_count(include_str!("input"));
        assert_eq!(result, 0);
    }
}
