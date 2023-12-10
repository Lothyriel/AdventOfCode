use crate::{get_steps, parse};

pub fn get_steps_count(input: &str) -> usize {
    let (directions, nodes) = parse(input);

    get_steps(&directions, "AAA", &nodes)
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
