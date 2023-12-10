use crate::PipeMaze;

pub fn get_max_distance(input: &str) -> usize {
    let maze = PipeMaze::parse(input);

    maze.farthest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_max_distance(include_str!("example"));
        assert_eq!(result, 4);
    }

    #[test]
    fn puzzle() {
        let result = get_max_distance(include_str!("input"));
        assert_eq!(result, 0);
    }
}
