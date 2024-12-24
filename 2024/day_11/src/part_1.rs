use crate::{blink_times, parse};

pub fn plutonian_pebbles(input: &str) -> usize {
    let mut stones = parse(input);

    blink_times(&mut stones, 25);

    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "125 17";
        let result = plutonian_pebbles(input);
        assert_eq!(55312, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = plutonian_pebbles(input);
        assert_eq!(203228, result);
    }
}
