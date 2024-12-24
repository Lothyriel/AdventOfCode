use crate::parse;

pub fn claw_contraption(input: &str) -> isize {
    parse(input).iter().filter_map(|c| c.min_tokens()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

        let result = claw_contraption(input);
        assert_eq!(480, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = claw_contraption(input);
        assert_eq!(36250, result);
    }
}
