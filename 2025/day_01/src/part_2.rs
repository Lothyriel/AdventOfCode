use crate::{Dial, parse};

pub fn day_x(input: &str) -> isize {
    let mut dial = Dial::new();

    parse(input).map(|r| dial.apply_get_wrap_count(&r)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        let result = day_x(input);
        assert_eq!(6, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = day_x(input);
        assert_eq!(6770, result);
    }
}
