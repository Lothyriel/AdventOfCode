use std::collections::HashMap;

use crate::{parse, Direction};

pub fn get_steps_count(input: &str) -> usize {
    let (directions, nodes) = parse(input);

    nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|n| get_steps(&directions, n, &nodes))
        .fold(1, lcm)
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn get_steps<'a>(d: &[Direction], mut c: &'a str, n: &'a HashMap<&str, (&str, &str)>) -> usize {
    for (steps, direction) in d.iter().cycle().enumerate() {
        if c.ends_with('Z') {
            return steps;
        }

        let next = n.get(c).expect("Should have this node");

        c = match direction {
            Direction::Left => next.0,
            Direction::Right => next.1,
        };
    }

    unreachable!("Cycled iteration shouldn't finish")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_steps_count(include_str!("example_part_2"));
        assert_eq!(result, 6);
    }

    #[test]
    fn puzzle() {
        let result = get_steps_count(include_str!("input"));
        assert_eq!(result, 21165830176709);
    }
}
