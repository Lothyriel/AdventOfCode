use crate::{add, mul, parse, valid_equation};

pub fn bridge_repair(input: &str) -> usize {
    let operators = [mul, add];

    parse(input)
        .iter()
        .filter(|(target, numbers)| valid_equation(*target, numbers, &operators))
        .map(|e| e.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        let result = bridge_repair(input);
        assert_eq!(3749, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = bridge_repair(input);
        assert_eq!(7710205485870, result);
    }
}
