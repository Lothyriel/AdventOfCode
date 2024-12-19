use crate::{get_minimum_steps, parse};

pub fn ram_run(input: &str, grid_size: usize) -> (usize, usize) {
    let bytes = parse(input);

    let mut answer = None;

    for (i, byte) in bytes.iter().enumerate() {
        if get_minimum_steps(&bytes, grid_size, i).is_some() {
            answer = Some(*byte);
        } else {
            return answer.expect("Valid answer");
        }
    }

    panic!("Should have a valid solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

        assert_eq!((6, 1), ram_run(input, 7));
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        assert_eq!((46, 18), ram_run(input, 71));
    }
}
