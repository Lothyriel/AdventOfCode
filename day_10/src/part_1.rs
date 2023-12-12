use crate::PipeMaze;

pub fn get_max_distance(input: &str, debug: bool) -> usize {
    let maze = PipeMaze::parse(input);

    maze.farthest(debug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_max_distance(include_str!("example"), false);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let result = get_max_distance(include_str!("example_2"), false);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_reddit() {
        let result = get_max_distance(include_str!("input_reddit"), false);
        assert_eq!(result, 3022);
    }

    #[test]
    fn puzzle() {
        let result = get_max_distance(include_str!("input"), false);
        assert_eq!(result, 0);
    }
}
