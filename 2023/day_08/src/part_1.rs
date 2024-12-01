use crate::Direction;

pub fn get_steps_count(input: &str) -> usize {
    let (directions, nodes) = crate::parse(input);

    let mut current = "AAA";

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
        assert_eq!(result, 21409);
    }
}
